use std::{error, fmt};

/// PGFPlots compatibility errors.
#[derive(Debug, Clone)]
pub enum PgfPlotsCompatError {
    /// Compatibility version does not exists.
    BadCompatVersion(String),
}

impl fmt::Display for PgfPlotsCompatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadCompatVersion(version) => write!(
                f,
                "pgfplots compatibility version `{version}` does not exists; \
                    available values are: {}",
                VERSIONS.join(", ")
            ),
        }
    }
}

impl error::Error for PgfPlotsCompatError {}

/// Available PGFPlots compatibility versions.
const VERSIONS: [&str; 19] = [
    "1.18", "1.17", "1.16", "1.15", "1.14", "1.13", "1.12", "1.11", "1.10", "1.9", "1.8", "1.7",
    "1.6", "1.5.1", "1.5", "1.4", "1.3", "pre1.3", "default",
];

/// PGFPlots compatibility layer.
#[derive(Debug, Clone)]
pub struct PgfPlotsCompat {
    version: String,
}

impl TryFrom<&str> for PgfPlotsCompat {
    type Error = PgfPlotsCompatError;

    fn try_from(version: &str) -> Result<Self, Self::Error> {
        Self::new(version)
    }
}

impl fmt::Display for PgfPlotsCompat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\pgfplotsset{{compat={}}}", self.version)
    }
}

impl Default for PgfPlotsCompat {
    fn default() -> Self {
        Self {
            version: "default".to_string(),
        }
    }
}

impl PgfPlotsCompat {
    /// Construct a new [`PgfPlotsCompat`], checking wheter the version is valid.
    fn new(version: &str) -> Result<Self, PgfPlotsCompatError> {
        if !VERSIONS.contains(&version) {
            return Err(PgfPlotsCompatError::BadCompatVersion(version.to_string()));
        }

        Ok(Self {
            version: version.to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compat_error() {
        assert!(PgfPlotsCompat::try_from("3.0").is_err());
        assert!(PgfPlotsCompat::try_from("1.18").is_ok());
        assert!(PgfPlotsCompat::try_from("default").is_ok());
    }

    #[test]
    fn display() {
        assert_eq!(
            r#"\pgfplotsset{compat=default}"#,
            PgfPlotsCompat::default().to_string()
        );
        assert_eq!(
            r#"\pgfplotsset{compat=1.18}"#,
            PgfPlotsCompat::try_from("1.18").unwrap().to_string()
        );
    }
}
