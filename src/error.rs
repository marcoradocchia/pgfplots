use crate::{
    document::preamble::PgfPlotsCompatError, engine::LatexEngine, output::LatexOutputSaveError,
};
use std::{error, fmt, io, process};
#[cfg(feature = "tectonic")]
use tectonic::errors::Error as TectonicError;

/// PGFPlots library errors.
#[derive(Debug)]
pub enum PgfPlotsError {
    Compile(CompileError),
    Show(opener::OpenError),
    Save(LatexOutputSaveError),
    Compat(PgfPlotsCompatError),
}

impl fmt::Display for PgfPlotsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compile(error) => write!(f, "compilation failed: {error}"),
            Self::Show(error) => write!(f, "unalbe to open produced output: {error}"),
            Self::Save(error) => write!(f, "unable to save produced output: {error}"),
            Self::Compat(error) => write!(f, "compat version: {error}"),
        }
    }
}

impl error::Error for PgfPlotsError {}

impl From<CompileError> for PgfPlotsError {
    fn from(error: CompileError) -> Self {
        Self::Compile(error)
    }
}

impl From<opener::OpenError> for PgfPlotsError {
    fn from(error: opener::OpenError) -> Self {
        Self::Show(error)
    }
}

impl From<PgfPlotsCompatError> for PgfPlotsError {
    fn from(error: PgfPlotsCompatError) -> Self {
        Self::Compat(error)
    }
}

impl From<LatexOutputSaveError> for PgfPlotsError {
    fn from(error: LatexOutputSaveError) -> Self {
        Self::Save(error)
    }
}

/// The error type returned when a [`Picture`] fails to compile into a PDF.
#[derive(Debug)]
pub enum CompileError {
    /// Temp directory error.
    TempDir(io::Error),
    /// I/O error.
    IO(io::Error),
    /// Compilation was executed but returned a non-zero exit code.
    BadExitStatus {
        /// LaTeX engine used.
        engine: LatexEngine,
        /// Compilation exit status.
        exit_status: process::ExitStatus,
        // TODO: LaTeX compilation errors.
    },
    #[cfg(feature = "tectonic")]
    /// Tectonic error.
    Tectonic(TectonicError),
}

impl From<io::Error> for CompileError {
    fn from(error: io::Error) -> Self {
        Self::IO(error)
    }
}

#[cfg(feature = "tectonic")]
impl From<TectonicError> for CompileError {
    fn from(error: TectonicError) -> Self {
        Self::Tectonic(error)
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TempDir(error) => write!(f, "tmp directory: {error}"),
            Self::IO(error) => write!(f, "I/O: {error}"),
            Self::BadExitStatus {
                engine,
                exit_status,
            } => write!(
                f,
                "`{engine}` LaTeX compiler exited with non-zero exit code: {exit_status}"
            ),
            #[cfg(feature = "tectonic")]
            Self::Tectonic(error) => write!(f, "tectonic engine: {error}"),
        }
    }
}

impl error::Error for CompileError {}
