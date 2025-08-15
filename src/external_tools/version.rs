use regex::Regex;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Version {
    pub major: u8,
    pub minor: Option<u8>,
    pub patch: Option<u8>,
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
                .map_err(|e| VersionParsingError::ParseIntError(e))
        })?;

        let minor: Option<u8> = version_capture.get(2).map_or(Ok(None), |v| {
            v.as_str()
                .parse()
                .map_err(|e| VersionParsingError::ParseIntError(e))
                .map(|v| Some(v))
        })?;

        let patch: Option<u8> = version_capture.get(3).map_or(Ok(None), |v| {
            v.as_str()
                .parse()
                .map_err(|e| VersionParsingError::ParseIntError(e))
                .map(|v| Some(v))
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
}
