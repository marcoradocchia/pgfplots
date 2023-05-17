use std::{error, fmt, str::FromStr};

/// Latex engine error.
#[derive(Debug)]
pub enum LatexEngineError {
    /// No such latex engine exists matching given name.
    NoSuchEngine(String),
}

impl fmt::Display for LatexEngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LatexEngineError::NoSuchEngine(invalid_engine_name) => write!(
                f,
                "no such latex engine exists with name `{invalid_engine_name}`"
            ),
        }
    }
}

impl error::Error for LatexEngineError {}

/// LaTeX Engine to compile a [`crate::document::Document`] into a PDF.
#[derive(Debug, Default, Clone, Copy)]
#[non_exhaustive]
pub enum LatexEngine {
    /// **Pdflatex** engine (requires `pdflatex` to be installed; included with TeXLive).
    PdfLatex,
    /// **LuaLatex** engine (requires `lualatex` to be installed; included with TeXLive).
    ///
    /// # Note
    /// From PGFPlots documentation: "*uses dynamic memory allocation such that it
    /// usually has enough memory for any pgfplots axis*".
    ///
    /// Hence the use of this engine is preferable in cases where memory is a limitation.
    /// Also, "*lualatex is supposed to be almost compatible with pdflatex*".
    #[default]
    LuaLatex,
    #[cfg(feature = "tectonic")]
    /// **Tectonic** engine (does not require any external software).
    Tectonic,
}

impl fmt::Display for LatexEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::PdfLatex => "pdflatex",
            Self::LuaLatex => "lualatex",
            #[cfg(feature = "tectonic")]
            Self::Tectonic => "tectonic",
        })
    }
}

impl FromStr for LatexEngine {
    type Err = LatexEngineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pdflatex" => Ok(Self::PdfLatex),
            "lualatex" => Ok(Self::LuaLatex),
            #[cfg(feature = "tectonic")]
            "tectonic" => Ok(Self::Tectonic),
            _ => Err(LatexEngineError::NoSuchEngine(s.to_string())),
        }
    }
}

impl LatexEngine {
    /// Returns [`LatexEngine`] CLI arguments.
    pub(crate) fn args(&self) -> [&str; 2] {
        match self {
            LatexEngine::PdfLatex => ["-interaction=batchmode", "-halt-on-error"],
            LatexEngine::LuaLatex => ["--interaction=batchmode", "--halt-on-error"],
            #[cfg(feature = "tectonic")]
            LatexEngine::Tectonic => unreachable!(),
        }
    }
}
