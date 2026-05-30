use crate::args::ARGS;
use linkify::{LinkFinder, LinkKind};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use qrcode_generator::QrCodeEcc;
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::Pasta;

use super::animalnumbers::to_u64;
use super::db::delete;
use super::hashids::to_u64 as hashid_to_u64;

pub fn remove_expired(pastas: &mut Vec<Pasta>) {
    let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => {
            log::error!("SystemTime before UNIX EPOCH!");
            0
        }
    } as i64;

    let ninety_days: i64 = 90 * 24 * 60 * 60;

    // Phase 1: mark newly-expired pastas; delete their files from disk but keep the record
    for p in pastas.iter_mut() {
        if p.expired {
            continue;
        }
        let should_expire = (p.expiration != 0 && p.expiration <= timenow)
            || (p.burn_after_reads > 0 && p.read_count >= p.burn_after_reads)
            || (ARGS.gc_days > 0 && p.last_read_days_ago() >= ARGS.gc_days);

        if should_expire {
            p.expired = true;
            p.expired_at = timenow;
            // Delete the attachment from disk; the pasta record stays for admin review
            if let Some(file) = &p.file {
                if fs::remove_file(format!(
                    "{}/attachments/{}/{}",
                    ARGS.data_dir, p.id_as_animals(), file.name()
                ))
                .is_err()
                {
                    log::error!("Failed to delete expired file {}!", file.name());
                }
                if fs::remove_dir(format!(
                    "{}/attachments/{}/",
                    ARGS.data_dir, p.id_as_animals()
                ))
                .is_err()
                {
                    log::error!("Failed to delete expired dir for {}!", file.name());
                }
            }
        }
    }

    // Phase 2: hard-delete records that have been expired more than 90 days
    pastas.retain(|p| {
        if p.expired && timenow - p.expired_at > ninety_days {
            delete(None, Some(p.id));
            false
        } else {
            true
        }
    });
}

/// Hard-delete all expired pasta records immediately (admin prune action).
pub fn prune_all_expired(pastas: &mut Vec<Pasta>) {
    pastas.retain(|p| {
        if p.expired {
            delete(None, Some(p.id));
            false
        } else {
            true
        }
    });
}

pub fn string_to_qr_svg(str: &str) -> String {
    qrcode_generator::to_svg_to_string(str, QrCodeEcc::Low, 256, None::<&str>).unwrap()
}

pub fn is_valid_url(url: &str) -> bool {
    let finder = LinkFinder::new();
    let spans: Vec<_> = finder.spans(url).collect();
    spans[0].as_str() == url && Some(&LinkKind::Url) == spans[0].kind()
}

pub fn encrypt(text_str: &str, key_str: &str) -> String {
    if text_str.is_empty() {
        return String::from("");
    }

    let mc = new_magic_crypt!(key_str, 256);

    mc.encrypt_str_to_base64(text_str)
}

pub fn decrypt(text_str: &str, key_str: &str) -> Result<String, magic_crypt::MagicCryptError> {
    if text_str.is_empty() {
        return Ok(String::from(""));
    }

    let mc = new_magic_crypt!(key_str, 256);

    mc.decrypt_base64_to_string(text_str)
}

pub fn encrypt_file(
    passphrase: &str,
    input_file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read the input file into memory
    let file = File::open(input_file_path).expect("Tried to encrypt non-existent file");
    let mut reader = BufReader::new(file);
    let mut input_data = Vec::new();
    reader.read_to_end(&mut input_data)?;

    // Create a MagicCrypt instance with the given passphrase
    let mc = new_magic_crypt!(passphrase, 256);

    // Encrypt the input data
    let ciphertext = mc.encrypt_bytes_to_bytes(&input_data[..]);

    // Write the encrypted data to a new file with the .enc extension
    let mut f = File::create(
        Path::new(input_file_path)
            .with_file_name("data")
            .with_extension("enc"),
    )?;
    f.write_all(ciphertext.as_slice())?;

    // Delete the original input file
    // input_file.seek(SeekFrom::Start(0))?;
    fs::remove_file(input_file_path)?;

    Ok(())
}

pub fn decrypt_file(
    passphrase: &str,
    input_file: &File,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Read the input file into memory
    let mut reader = BufReader::new(input_file);
    let mut ciphertext = Vec::new();
    reader.read_to_end(&mut ciphertext)?;

    // Create a MagicCrypt instance with the given passphrase
    let mc = new_magic_crypt!(passphrase, 256);
    // Encrypt the input data
    let res = mc.decrypt_bytes_to_bytes(&ciphertext[..]);

    if res.is_err() {
        return Err("Failed to decrypt file".into());
    }

    Ok(res.unwrap())
}

/// Find a pasta by slug (either custom URL or generated ID)
/// Returns the index of the pasta if found, None otherwise
pub fn find_pasta_by_slug(pastas: &[Pasta], slug: &str) -> Option<usize> {
    // Try to find by custom URL first
    for (i, pasta) in pastas.iter().enumerate() {
        if let Some(ref custom_url) = pasta.custom_url {
            if custom_url == slug {
                return Some(i);
            }
        }
    }

    // If not found by custom URL, try by generated ID
    let id = if ARGS.hash_ids {
        hashid_to_u64(slug).unwrap_or(0)
    } else {
        to_u64(slug).unwrap_or(0)
    };

    for (i, pasta) in pastas.iter().enumerate() {
        if pasta.id == id {
            return Some(i);
        }
    }

    None
}
