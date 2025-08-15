use regex::Regex;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct Version {
    pub major: u8,
    pub minor: Option<u8>,
    pub patch: Option<u8>,
}

impl Version {
    fn matches(&self, other: &Version) -> bool {
        if self.major != other.major {
            return false;
        }

        let minor_match = match (self.minor, other.minor) {
            (None, _) => true,
            (_, None) => true,
            (Some(a), Some(b)) => a == b,
        };

        if !minor_match {
            return false;
        }

        let patch_match = match (self.patch, other.patch) {
            (None, _) => true,
            (_, None) => true,
            (Some(a), Some(b)) => a == b,
        };

        patch_match
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.major)?;
        if let Some(minor) = self.minor {
            write!(f, ".{}", minor)?;
        }
        if let Some(patch) = self.patch {
            write!(f, ".{}", patch)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum VersionParsingError {
    ParseIntError(std::num::ParseIntError),
    VersionNotFound,
}

impl FromStr for Version {
    type Err = VersionParsingError;

    /// reasonably "smart" from_str that can extract version from "arbitrary" string
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version_regex = Regex::new(r"(\d+)\.?(\d+)?\.?(\d+)?").unwrap();
        let version_capture = version_regex
            .captures(s)
            .ok_or(VersionParsingError::VersionNotFound)?;

        let major: u8 = version_capture.get(1).map_or(Ok(0), |v| {
            v.as_str()
                .parse()
                .map_err(VersionParsingError::ParseIntError)
        })?;

        let minor: Option<u8> = version_capture.get(2).map_or(Ok(None), |v| {
            v.as_str()
                .parse()
                .map_err(VersionParsingError::ParseIntError)
                .map(Some)
        })?;

        let patch: Option<u8> = version_capture.get(3).map_or(Ok(None), |v| {
            v.as_str()
                .parse()
                .map_err(VersionParsingError::ParseIntError)
                .map(Some)
        })?;

        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version_from_string() {
        let version: Version = "3.13.2".parse().unwrap();
        assert_eq!(
            version,
            Version {
                major: 3,
                minor: Some(13),
                patch: Some(2)
            }
        );
    }

    #[test]
    fn test_parse_version_from_string_without_patch() {
        let version: Version = "123.45".parse().unwrap();
        assert_eq!(
            version,
            Version {
                major: 123,
                minor: Some(45),
                patch: None
            }
        );
    }

    #[test]
    fn test_parse_version_from_string_without_patch_and_minor() {
        let version: Version = "123".parse().unwrap();
        assert_eq!(
            version,
            Version {
                major: 123,
                minor: None,
                patch: None
            }
        );
    }

    #[test]
    fn test_parse_version_from_string_with_additional_version_fields() {
        let version: Version = "123.45.6.789".parse().unwrap();
        assert_eq!(
            version,
            Version {
                major: 123,
                minor: Some(45),
                patch: Some(6)
            }
        );
    }

    #[test]
    fn test_parse_version_from_string_invalid() {
        let version = "invalid".parse::<Version>();
        assert!(version.is_err());
    }

    #[test]
    fn test_version_to_string() {
        let version = Version {
            major: 3,
            minor: Some(13),
            patch: Some(2),
        };
        assert_eq!(version.to_string(), "3.13.2");
    }

    #[test]
    fn test_version_to_string_without_patch() {
        let version = Version {
            major: 3,
            minor: Some(13),
            patch: None,
        };
        assert_eq!(version.to_string(), "3.13");
    }

    #[test]
    fn test_version_to_string_without_patch_and_minor() {
        let version = Version {
            major: 3,
            minor: None,
            patch: None,
        };
        assert_eq!(version.to_string(), "3");
    }

    #[test]
    fn test_version_matches_exactly() {
        let first = Version {
            major: 1,
            minor: Some(2),
            patch: Some(3),
        };

        let second = Version {
            major: 1,
            minor: Some(2),
            patch: Some(3),
        };

        assert!(first.matches(&second));
    }

    #[test]
    fn test_version_does_not_match_exactly_on_different_patch() {
        let first = Version {
            major: 1,
            minor: Some(2),
            patch: Some(3),
        };

        let second = Version {
            major: 1,
            minor: Some(2),
            patch: Some(4),
        };

        assert!(!first.matches(&second));
    }
    
    #[test]
    fn test_version_does_not_match_exactly_on_different_minor() {
        let first = Version {
            major: 1,
            minor: Some(2),
            patch: Some(3),
        };

        let second = Version {
            major: 1,
            minor: Some(4),
            patch: Some(3),
        };

        assert!(!first.matches(&second));
    }
    
    #[test]
    fn test_version_does_not_match_exactly_on_different_major() {
        let first = Version {
            major: 2,
            minor: Some(2),
            patch: Some(3),
        };

        let second = Version {
            major: 1,
            minor: Some(2),
            patch: Some(3),
        };

        assert!(!first.matches(&second));
    }

    #[test]
    fn test_version_matches_without_patch() {
        let first = Version {
            major: 1,
            minor: Some(2),
            patch: None,
        };

        let second = Version {
            major: 1,
            minor: Some(2),
            patch: Some(3),
        };

        assert!(first.matches(&second));
    }
    
    #[test]
    fn test_version_matches_without_patch_when_both_none() {
        let first = Version {
            major: 1,
            minor: Some(2),
            patch: None,
        };

        let second = Version {
            major: 1,
            minor: Some(2),
            patch: None,
        };

        assert!(first.matches(&second));
    }
    
    #[test]
    fn test_version_matches_without_minor() {
        let first = Version {
            major: 1,
            minor: Some(2),
            patch: None,
        };

        let second = Version {
            major: 1,
            minor: None,
            patch: None,
        };

        assert!(first.matches(&second));
    }
    
    #[test]
    fn test_version_matches_without_minor_when_both_none() {
        let first = Version {
            major: 1,
            minor: None,
            patch: None,
        };

        let second = Version {
            major: 1,
            minor: None,
            patch: None,
        };

        assert!(first.matches(&second));
    }
}
