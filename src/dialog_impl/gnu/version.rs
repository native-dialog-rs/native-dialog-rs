use std::cmp::Ordering;
use versions::SemVer;

#[derive(Debug)]
pub struct Ver(SemVer);

impl Ver {
    pub fn new(s: &str) -> Option<Self> {
        let semver = SemVer::new(s.trim())?;
        Some(Self(semver))
    }
}

impl PartialEq<(u32, u32, u32)> for Ver {
    fn eq(&self, other: &(u32, u32, u32)) -> bool {
        let other = SemVer {
            major: other.0,
            minor: other.1,
            patch: other.2,
            pre_rel: None,
            meta: None,
        };
        self.0 == other
    }
}

impl PartialOrd<(u32, u32, u32)> for Ver {
    fn partial_cmp(&self, other: &(u32, u32, u32)) -> Option<Ordering> {
        let other = SemVer {
            major: other.0,
            minor: other.1,
            patch: other.2,
            pre_rel: None,
            meta: None,
        };
        self.0.partial_cmp(&other)
    }
}
