use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub title: Cow<'static, str>,
    pub long_title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub date: Cow<'static, str>,
    pub update_type: Cow<'static, str>,
}

pub static CURRENT_VERSION: Version = Version {
    major: 2,
    minor: 1,
    patch: 3,
    title: Cow::Borrowed(env!("CARGO_PKG_VERSION")),
    long_title: Cow::Borrowed(concat!("Version ", env!("CARGO_PKG_VERSION"))),
    description: Cow::Borrowed("w3K fork: custom URL slugs, HTML/Markdown inline rendering, CSP hardening, native ARM builds."),
    date: Cow::Borrowed("2026-05-30"),
    update_type: Cow::Borrowed("stable"),
};

impl Version {
    pub fn newer_than(&self, other: &Version) -> bool {
        if self.major != other.major {
            self.major > other.major
        } else if self.minor != other.minor {
            self.minor > other.minor
        } else {
            self.patch > other.patch
        }
    }

    pub fn newer_than_current(&self) -> bool {
        self.newer_than(&CURRENT_VERSION)
    }
}

pub async fn fetch_latest_version() -> Result<Version, reqwest::Error> {
    let url = "https://api.microbin.eu/version/";
    let http_client = crate::util::http_client::new_async();
    let response = http_client.get(url).send().await?;
    let version = response.json::<Version>().await?;

    Ok(version)
}
