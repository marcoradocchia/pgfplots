mod compat;
mod package;
mod pgfplotslib;

pub use compat::{PgfPlotsCompat, PgfPlotsCompatError};
pub use package::Package;
pub use pgfplotslib::PgfPlotsLib;
use std::fmt;

/// LaTeX document preamble.
#[derive(Debug, Default, Clone)]
pub struct Preamble {
    /// LaTeX packages to include in the document compilation.
    pkgs: Vec<Package>,
    /// PGFPlots libraries which need to be activeated separately.
    pgflibs: Vec<PgfPlotsLib>,
    /// PGFPlots compatibility layer.
    pgfcompat: PgfPlotsCompat,
}

impl From<PgfPlotsCompat> for Preamble {
    fn from(pgfcompat: PgfPlotsCompat) -> Self {
        Self {
            pkgs: vec![],
            pgflibs: vec![],
            pgfcompat,
        }
    }
}

impl Preamble {
    /// Construct a new [`Preamble`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Construct a new [`Preamble`] with specific PGFPlots compatibility layer version.
    pub fn with_pgfcompat_version(version: &str) -> Result<Self, PgfPlotsCompatError> {
        Ok(Self {
            pkgs: vec![],
            pgflibs: vec![],
            pgfcompat: PgfPlotsCompat::try_from(version)?,
        })
    }

    /// Set PGFPlots compatibility layer.
    pub fn pgfcompat(mut self, pgfcompat: PgfPlotsCompat) -> Self {
        self.pgfcompat = pgfcompat;
        self
    }

    /// Set PGFPlots compatibility layer version.
    pub fn pgfcompat_version(mut self, version: &str) -> Result<Self, PgfPlotsCompatError> {
        self.pgfcompat = PgfPlotsCompat::try_from(version)?;
        Ok(self)
    }

    /// Add a PGFPlots library to the document preamble.
    pub fn pgflib(mut self, lib: PgfPlotsLib) -> Self {
        self.pgflibs.push(lib);
        self
    }

    /// Add a PGFPlots libraries to the document preamble.
    pub fn pgflibs(mut self, libs: &[PgfPlotsLib]) -> Self {
        self.pgflibs.extend_from_slice(libs);
        self
    }

    /// Add a LaTeX package to the document preamble.
    pub fn pkg(mut self, pkg: Package) -> Self {
        self.pkgs.push(pkg);
        self
    }

    /// Add LaTeX packages to the document preamble.
    pub fn pkgs(mut self, pkgs: &[Package]) -> Self {
        self.pkgs.extend_from_slice(pkgs);
        self
    }

    /// Set PGFPlots compatibility layer.
    pub fn set_pgfcompat(&mut self, pgfcompat: PgfPlotsCompat) {
        self.pgfcompat = pgfcompat;
    }

    /// Set PGFPlots compatibility layer version.
    pub fn set_pgfcompat_version(&mut self, version: &str) -> Result<(), PgfPlotsCompatError> {
        self.pgfcompat = PgfPlotsCompat::try_from(version)?;

        Ok(())
    }

    /// Add a PGFPlots library to the document preamble.
    pub fn add_pgflib(&mut self, lib: PgfPlotsLib) {
        self.pgflibs.push(lib);
    }

    /// Add a PGFPlots libraries to the document preamble.
    pub fn add_pgflibs(&mut self, libs: &[PgfPlotsLib]) {
        self.pgflibs.extend_from_slice(libs);
    }

    /// Add a LaTeX package to the document preamble.
    pub fn add_pkg(&mut self, pkg: Package) {
        self.pkgs.push(pkg);
    }

    /// Add LaTeX packages to the document preamble.
    pub fn add_pkgs(&mut self, pkgs: &[Package]) {
        self.pkgs.extend_from_slice(pkgs);
    }
}

impl fmt::Display for Preamble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "\\documentclass{{standalone}}\n\\usepackage{{pgfplots}}\n{}",
            self.pgfcompat
        )?;

        // Add PGFPlots libraries one per line.
        for pgflib in &self.pgflibs {
            writeln!(f, "{pgflib}")?;
        }

        // Add LaTeX packages one per line.
        for pkg in &self.pkgs {
            writeln!(f, "{pkg}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn preamble() {
        let preamble = Preamble::new()
            .pkg(Package::new("babel", &["italian"]))
            .pkg("braket".into());
        assert_eq!(
            r#"\documentclass{standalone}
\usepackage{pgfplots}
\pgfplotsset{compat=default}
\usepackage{babel}[italian]
\usepackage{braket}
"#,
            preamble.to_string()
        );
    }

    #[test]
    fn package() {
        let package = Package::new("babel", &["italian"]);
        assert_eq!("\\usepackage{babel}[italian]", package.to_string());

        let mut package: Package = "babel".into();
        assert_eq!("\\usepackage{babel}", package.to_string());

        package.add_option("italian");
        assert_eq!("\\usepackage{babel}[italian]", package.to_string());

        package.add_option("english");
        assert_eq!("\\usepackage{babel}[italian, english]", package.to_string());
    }
}
