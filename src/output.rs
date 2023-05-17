use crate::{engine::LatexEngine, error::CompileError, Result};
use std::{
    error, fmt, fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    result,
};
use tempfile::{Builder as TmpBuilder, TempDir};

/// LaTeX output save error.
#[derive(Debug)]
pub enum LatexOutputSaveError {
    /// Unable to create destination directory.
    CreateDestDir(io::Error),
    /// Unable to access destination path due to denied permission.
    PermissionDenied(io::Error),
    /// Passed invalid save path.
    InvalidPath(PathBuf),
    /// Unable to save file to destination directory.
    SaveFail(PathBuf, io::Error),
    /// Other unmanaged errors.
    Other(io::Error),
}

impl fmt::Display for LatexOutputSaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateDestDir(error) => {
                write!(f, "unable to create destination directory: {error}")
            }
            Self::PermissionDenied(error) => {
                write!(f, "unable to access destination path: {error}")
            }
            Self::InvalidPath(path) => write!(f, "invalid save path: {}", path.display()),
            Self::SaveFail(path, error) => write!(
                f,
                "unable to save to destination '{}': {error}",
                path.display()
            ),
            Self::Other(error) => write!(f, "{error}"),
        }
    }
}

impl error::Error for LatexOutputSaveError {}

impl From<io::Error> for LatexOutputSaveError {
    fn from(error: io::Error) -> Self {
        Self::CreateDestDir(error)
    }
}

/// LaTeX document output type.
#[derive(Debug, Clone, Copy, Default)]
pub enum LatexOutputType {
    #[default]
    Pdf,
}

impl fmt::Display for LatexOutputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Pdf => "pdf",
        })
    }
}

impl LatexOutputType {
    /// Returns output type file extension.
    fn ext(&self) -> String {
        self.to_string()
    }
}

/// [`LatexOutput`] factory.
struct LatexOutputBuilder<'a, 'b, 's> {
    /// LaTeX output type.
    output_type: LatexOutputType,
    /// Output file stem.
    file_stem: &'s str,
    /// Custom temporary output directory:
    /// if [`None`] it will be automatically set.
    dir: Option<TmpBuilder<'a, 'b>>,
}

impl<'a, 'b, 's> Default for LatexOutputBuilder<'a, 'b, 's> {
    fn default() -> Self {
        Self {
            output_type: Default::default(),
            file_stem: "pgfplot",
            dir: None,
        }
    }
}

impl<'a, 'b, 's> LatexOutputBuilder<'a, 'b, 's> {
    /// Sets the [`LatexOutput`]'s [`LatexOutputType`].
    fn output_type(&mut self, output_type: LatexOutputType) -> &mut Self {
        self.output_type = output_type;
        self
    }

    /// Sets the [`LatexOutput`]'s file stem.
    fn file_stem<S>(&mut self, file_stem: S) -> &mut Self
    where
        S: AsRef<&'s str>,
    {
        self.file_stem = file_stem.as_ref();
        self
    }

    /// Sets the [`LatexOutput`]'s directory.
    fn dir(&mut self, dir: TmpBuilder<'a, 'b>) -> &mut Self {
        self.dir = Some(dir);
        self
    }

    /// Builds the [`LatexOutput`], consuming the [`LatexOutputBuilder`].
    fn build(self) -> Result<LatexOutput> {
        let dir = self
            .dir
            .unwrap_or({
                let mut builder = TmpBuilder::new();
                builder.prefix("pgfplot");
                builder
            })
            .tempdir()
            .map_err(CompileError::TempDir)?;

        Ok(LatexOutput {
            output_type: self.output_type,
            tex_file: dir.path().join(format!("{}.tex", self.file_stem)),
            dir,
        })
    }
}

/// LaTeX document output.
#[derive(Debug)]
pub struct LatexOutput {
    /// Output type.
    output_type: LatexOutputType,
    /// Main Tex file.
    tex_file: PathBuf,
    /// Ouptut temporary directory.
    dir: TempDir,
}

impl LatexOutput {
    /// Retuns a [`LatexOutputBuilder`] factory.
    #[inline]
    fn builder<'a, 'b, 's>() -> LatexOutputBuilder<'a, 'b, 's> {
        LatexOutputBuilder::default()
    }

    /// Constructs a new [`LatexOutput`] using defaults.
    pub(crate) fn new() -> Result<Self> {
        Self::builder().build()
    }

    /// Retuns a reference to [`LatexOutput`]'s directory path.
    #[inline]
    fn dir_path(&self) -> &Path {
        self.dir.path()
    }

    /// Returns a reference to main Tex [`LatexOutput`]'s file pathbuf.
    #[inline]
    fn tex_file_path(&self) -> &Path {
        self.tex_file.as_path()
    }

    /// Returns a reference to the produced output file path.
    pub(crate) fn output_path(&self) -> PathBuf {
        self.tex_file.with_extension(self.output_type.ext())
    }

    /// Compile LaTeX output using Tectonic engine.
    #[cfg(feature = "tectonic")]
    fn compile_tectonic(&self) -> Result<(), CompileError> {
        // Modified from `tectonic::latex_to_pdf` to generate the files
        // instead of just returning the bytes.

        let mut status = tectonic::status::NoopStatusBackend::default();

        let auto_create_config_file = false;
        let config = tectonic::ctry!(
            tectonic::config::PersistentConfig::open(auto_create_config_file);
            "failed to open the default configuration file"
        );

        let only_cached = false;
        let bundle = tectonic::ctry!(
            config.default_bundle(only_cached, &mut status);
            "failed to load the default resource bundle"
        );

        let format_cache_path = tectonic::ctry!(
            config.format_cache_path();
            "failed to set up the format cache"
        );

        let mut sb = tectonic::driver::ProcessingSessionBuilder::default();
        sb.bundle(bundle)
            .primary_input_path(self.tex_file_path())
            .tex_input_name(self.tex_file_path().file_name().unwrap())
            .format_name("latex")
            .format_cache_path(format_cache_path)
            .keep_logs(true) // Just to keep the behaviour consistent with `pdflatex`
            .keep_intermediates(true)
            .print_stdout(false)
            .output_format(tectonic::driver::OutputFormat::Pdf)
            .output_dir(self.dir_path());

        let mut sess = tectonic::ctry!(
            sb.create(&mut status);
            "failed to initialize the LaTeX processing session"
        );
        tectonic::ctry!(
            sess.run(&mut status);
            "`tectonic` LaTeX engine failed"
        );
    }

    /// Compile LaTeX output.
    pub(crate) fn compile<S>(
        &self,
        engine: LatexEngine,
        source: S,
    ) -> result::Result<(), CompileError>
    where
        S: AsRef<str>,
    {
        #[cfg(feature = "tectonic")]
        if engine == LatexEngine::Tectonic {
            return self.compile_tectonic();
        }

        fs::File::create(self.tex_file_path())?.write_all(source.as_ref().as_bytes())?;

        let exit_status = Command::new(engine.to_string())
            .current_dir(self.dir_path())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .args(engine.args())
            .arg(self.tex_file_path())
            .status()?;

        if !exit_status.success() {
            return Err(CompileError::BadExitStatus {
                engine,
                exit_status,
            });
        }

        Ok(())
    }

    /// Saves LaTeX output to specified `path` and returns whether the file has been saved.
    pub fn save<P, F>(&self, path: P, overwrite: F) -> Result<bool>
    where
        P: AsRef<Path>,
        F: FnOnce() -> Result<bool>,
    {
        let path = path.as_ref();
        let output_path = self.output_path();

        let copy = |path: &Path| -> Result<bool> {
            fs::copy(&output_path, path)
                .map_err(|error| LatexOutputSaveError::SaveFail(path.to_path_buf(), error))?;

            Ok(true)
        };

        match fs::metadata(path) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    copy(&path.join(output_path.file_name().unwrap()))
                } else if metadata.is_file() {
                    if overwrite()? {
                        copy(path)
                    } else {
                        Ok(false)
                    }
                } else {
                    unreachable!()
                }
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                let parent = path
                    .parent()
                    .ok_or_else(|| LatexOutputSaveError::InvalidPath(path.to_path_buf()))?;

                match fs::metadata(parent) {
                    Ok(parent_metadata) if parent_metadata.is_dir() => copy(path),
                    Err(error) if error.kind() == io::ErrorKind::NotFound => {
                        fs::create_dir_all(parent).map_err(LatexOutputSaveError::CreateDestDir)?;
                        copy(path)
                    }
                    Err(error) if error.kind() == io::ErrorKind::PermissionDenied => {
                        Err(LatexOutputSaveError::PermissionDenied(error).into())
                    }
                    Err(error) => Err(LatexOutputSaveError::Other(error).into()),
                    _ => unreachable!(),
                }
            }
            Err(error) if error.kind() == io::ErrorKind::PermissionDenied => {
                Err(LatexOutputSaveError::PermissionDenied(error).into())
            }
            Err(error) => Err(LatexOutputSaveError::Other(error).into()),
        }
    }

    /// Opens [`LatexOutput`] with the default system program.
    pub fn open(&self) -> Result<()> {
        opener::open(self.output_path())?;

        Ok(())
    }
}
