pub mod preamble;
pub mod tikzpicture;

use crate::{engine::LatexEngine, output::LatexOutput, Result};
use preamble::{Package, PgfPlotsCompat, PgfPlotsLib, Preamble};
use tikzpicture::{axis::Axis, TikzPicture};

/// Standalone LaTeX document used to generate the plot.
#[derive(Debug, Clone, Default)]
pub struct Document {
    /// LaTeX document *preamble*.
    preamble: Preamble,
    /// LaTeX document *body* (*pictures*).
    body: Vec<TikzPicture>,
}

impl Document {
    /// Constructs a new empty [`Document`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Set PGFPlots compatibility layer.
    pub fn pgfcompat(mut self, pgfcompat: PgfPlotsCompat) -> Self {
        self.preamble.set_pgfcompat(pgfcompat);
        self
    }

    /// Set PGFPlots compatibility layer version.
    pub fn pgfcompat_version(mut self, version: &str) -> Result<Self> {
        self.preamble.set_pgfcompat_version(version)?;
        Ok(self)
    }

    /// Add a PGFPlots library to the document preamble.
    pub fn pgflib<L>(mut self, lib: L) -> Self
    where
        L: Into<PgfPlotsLib>,
    {
        self.preamble.add_pgflib(lib.into());
        self
    }

    /// Add a PGFPlots libraries to the document preamble.
    pub fn pgflibs(mut self, libs: &[PgfPlotsLib]) -> Self {
        self.preamble.add_pgflibs(libs);
        self
    }

    /// Add a LaTeX package to the document preamble.
    pub fn pkg<P>(mut self, pkg: P) -> Self
    where
        P: Into<Package>,
    {
        self.preamble.add_pkg(pkg.into());
        self
    }

    /// Add LaTeX packages to the document preamble.
    pub fn pkgs(mut self, pkgs: &[Package]) -> Self {
        self.preamble.add_pkgs(pkgs);
        self
    }

    /// Add a [`TikzPicture`] (equivalent to the Ti*k*Z graphics environment) to the document.
    pub fn picture<P>(mut self, tikzpicture: P) -> Self
    where
        P: Into<TikzPicture>,
    {
        let tikzpicture = tikzpicture.into();
        self.add_pgflibs(&tikzpicture.required_libs());
        self.body.push(tikzpicture);
        self
    }

    /// Set PGFPlots compatibility layer.
    pub fn set_pgfcompat<C>(&mut self, pgfcompat: C)
    where
        C: Into<PgfPlotsCompat>,
    {
        self.preamble.set_pgfcompat(pgfcompat.into());
    }

    /// Set PGFPlots compatibility layer version.
    pub fn set_pgfcompat_version(&mut self, version: &str) -> Result<()> {
        self.preamble.set_pgfcompat_version(version)?;

        Ok(())
    }

    /// Add a PGFPlots library to the document preamble.
    pub fn add_pgflib<L>(&mut self, lib: L)
    where
        L: Into<PgfPlotsLib>,
    {
        self.preamble.add_pgflib(lib.into());
    }

    /// Add a PGFPlots libraries to the document preamble.
    pub fn add_pgflibs(&mut self, libs: &[PgfPlotsLib]) {
        self.preamble.add_pgflibs(libs);
    }

    /// Add a LaTeX package to the document preamble.
    pub fn add_pkg<P>(&mut self, pkg: P)
    where
        P: Into<Package>,
    {
        self.preamble.add_pkg(pkg.into());
    }

    /// Add LaTeX packages to the document preamble.
    pub fn add_pkgs(&mut self, pkgs: &[Package]) {
        self.preamble.add_pkgs(pkgs);
    }

    /// Add a [`TikzPicture`] (equivalent to the Ti*k*Z graphics environment) to the document.
    pub fn add_picture<P>(&mut self, tikzpicture: P)
    where
        P: Into<TikzPicture>,
    {
        let tikzpicture = tikzpicture.into();
        self.add_pgflibs(&tikzpicture.required_libs());
        self.body.push(tikzpicture);
    }

    /// Return a [`String`] with valid LaTeX code that generates a standalone PDF.
    ///
    /// # Note
    ///
    /// Passing this string directly to e.g. `pdflatex` will fail to generate a
    /// PDF document. It is usually necessary to [`str::replace`] all the
    /// occurrences of `\n` and `\t` with white space before sending this string
    /// as an argument to a LaTeX compiler.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::{document::tikzpicture::TikzPicture, document::Document};
    ///
    /// let mut document = Document::new();
    /// document.add_picture(TikzPicture::new());
    /// assert_eq!(
    /// r#"\documentclass{standalone}
    /// \usepackage{pgfplots}
    ///
    /// \begin{document}
    /// \begin{tikzpicture}
    /// \end{tikzpicture}
    /// \end{document}"#,
    /// document.standalone_string());
    /// ```
    pub fn standalone_string(&self) -> String {
        [
            &self.preamble.to_string(),
            r"\begin{document}",
            &self
                .body
                .iter()
                .map(|picture| picture.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            r"\end{document}",
        ]
        .join("\n")
    }

    /// Compile the picture environment into a standalone PDF document. This
    /// will create a `pgfplot.pdf` file in the system temporary directory (e.g. `/tmp` on Linux
    /// systems). Additional files will be created in the same directory (e.g. `.log` and
    /// `.aux` files).
    pub fn pdf(&self, engine: LatexEngine) -> Result<LatexOutput> {
        // Copy the tex code to a temporary file instead of passing it directly
        // to the engine via e.g. stdin. This avoids the "Argument list too
        // long" error when there are e.g. too many points in a plot.
        let latex_output = LatexOutput::new()?;
        latex_output.compile(engine, self.standalone_string())?;

        Ok(latex_output)
    }
}

impl From<Preamble> for Document {
    fn from(preamble: Preamble) -> Self {
        Self {
            preamble,
            body: vec![],
        }
    }
}

impl From<TikzPicture> for Document {
    fn from(picture: TikzPicture) -> Self {
        let mut document = Self::new();
        document.add_picture(picture);
        document
    }
}

impl From<Axis> for Document {
    fn from(axis: Axis) -> Self {
        Self::from(TikzPicture::from(axis))
    }
}

#[cfg(test)]
mod test {
    // use super::*;
    //
    // #[test]
    // fn document() {
    //             let mut document = Document::new();
    //             document
    //                 .preamble
    //                 .add_pkg(Package::new("babel", &["italian"]));
    //             document.preamble.add_pkg("braket".into());
    //             assert_eq!(
    //                 r#"\documentclass{standalone}
    //     \usepackage{pgfplots}
    //     \usepackage{babel}[italian]
    //     \usepackage{braket}
    //
    //     \begin{document}
    //
    //     \end{document}"#,
    //                 document.standalone_string()
    //             );
    //
    //             document.add_picture(Picture::new());
    //             assert_eq!(
    //                 r#"\documentclass{standalone}
    //     \usepackage{pgfplots}
    //     \usepackage{babel}[italian]
    //     \usepackage{braket}
    //
    //     \begin{document}
    //     \begin{tikzpicture}
    //     \end{tikzpicture}
    //     \end{document}"#,
    //                 document.standalone_string()
    //             );
    // }
}
