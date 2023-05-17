pub mod axis;

use super::preamble::PgfPlotsLib;
use axis::Axis;
use std::fmt;

// /// Implementors of this trait represent types who can be used inside an
// /// [`crate::document::tikzpicture::TikzPicture`] environment.
// ///
// /// In LaTeX this translates to the usage of one environment which can be used inside the
// /// `tikzpicture` environment, like below:
// ///
// /// ```tex
// /// \begin{<envname>}[...]
// ///     ...
// /// \end{<envname>}
// /// ```
// pub trait TikzInnerEnv: fmt::Debug + fmt::Display + dyn_clone::DynClone {}
//
// // Allows `struct`s containing Box<dyn AddPlot> derive Clone.
// dyn_clone::clone_trait_object!(TikzInnerEnv);

#[derive(Debug, Clone)]
pub enum TikzInnerEnv {
    Axis(Axis),
}

impl From<Axis> for TikzInnerEnv {
    fn from(axis: Axis) -> Self {
        Self::Axis(axis)
    }
}

impl fmt::Display for TikzInnerEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Axis(env) => write!(f, "{env}"),
        }
    }
}

impl TikzInnerEnv {
    /// Returns a vector of [`PgfPlotsLib`]s required by each plots in the contained
    /// inner environment.
    fn required_libs(&self) -> Vec<PgfPlotsLib> {
        match self {
            Self::Axis(env) => env.required_libs(),
        }
    }
}

/// Ti*k*Z options passed to the [`TikzPicture`] environment.
///
/// The most commonly used key-value pairs are variants of the [`TikzPictureOption`]
/// enum. The [`TikzPictureOption::Custom`] variant is provided to add unimplemented
/// keys and will be written verbatim in the options of the [`TikzPicture`]
/// environment.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum TikzPictureOption {
    /// Custom key-value pairs that have not been implemented. These will be
    /// appended verbatim to the options of the [`Picture`].
    Custom(String),
}

impl fmt::Display for TikzPictureOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TikzPictureOption::Custom(key) => write!(f, "{key}"),
        }
    }
}

/// Ti*k*Z picture environment.
///
/// Creating a [`TikzPicture`] is equivalent to the Ti*k*Z graphics environment:
///
/// ```tex
/// \begin{tikzpicture}[TikzPictureOptions]
///     % axis environments
/// \end{tikzpicture}
/// ```
#[derive(Clone, Debug, Default)]
pub struct TikzPicture {
    options: Vec<TikzPictureOption>,
    inner_env: Vec<TikzInnerEnv>,
}

impl fmt::Display for TikzPicture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\begin{{tikzpicture}}")?;
        // If there are keys, print one per line. It makes it easier for a
        // human later to find keys if they are divided by lines.
        if !self.options.is_empty() {
            writeln!(f, "[")?;
            for key in self.options.iter() {
                writeln!(f, "\t{key},")?;
            }
            write!(f, "]")?;
        }
        writeln!(f)?;

        for axis in self.inner_env.iter() {
            writeln!(f, "{axis}")?;
        }

        write!(f, "\\end{{tikzpicture}}")?;

        Ok(())
    }
}

impl From<TikzInnerEnv> for TikzPicture {
    fn from(env: TikzInnerEnv) -> Self {
        Self {
            options: vec![],
            inner_env: vec![env],
        }
    }
}

impl From<Axis> for TikzPicture {
    fn from(axis: Axis) -> Self {
        Self::from(TikzInnerEnv::Axis(axis))
    }
}

impl TikzPicture {
    /// Create a new, empty picture environment.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::Picture;
    ///
    /// let picture = Picture::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a key to control the appearance of the picture. This will overwrite
    /// any previous mutually exclusive key.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::{TikzPicture, TikzPictureOption};
    ///
    /// let mut picture = Picture::new();
    /// picture.add_option(PictureKey::Custom(String::from("baseline")));
    /// ```
    pub fn add_option(&mut self, option: TikzPictureOption) {
        match option {
            TikzPictureOption::Custom(_) => (),
            // If/whenever another variant is added, handle it the same way as
            // Axis::add_key and Plot2D::add_key
        }
        self.options.push(option);
    }

    /// Returns a vector of required PGFPlots libraries based on the type of contained [`Plot`]s.
    pub fn required_libs(&self) -> Vec<PgfPlotsLib> {
        self.inner_env
            .iter()
            .flat_map(|env| env.required_libs())
            .collect()
    }

    /// Add a new [`TikzInnerEnv`] to the Ti*k*Z picture.
    pub fn add_env(&mut self, env: TikzInnerEnv) {
        self.inner_env.push(env);
    }

    /// Add a new [`Axis`] environment to the Ti*k*Z picture.
    pub fn add_axis(&mut self, axis: Axis) {
        self.inner_env.push(axis.into());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        document::tikzpicture::axis::Axis,
        libs::statistics::histogram::Histogram,
    };

    #[test]
    fn required_libs() {
        let plot = Histogram::new();
        let histogram = Histogram::new();

        let mut axis = Axis::new();
        axis.add_plot(plot.into());
        axis.add_plot(histogram.into());

        let picture = TikzPicture::from(axis);

        assert_eq!(
            [PgfPlotsLib::Statistics].as_slice(),
            &picture.required_libs()
        );
    }
}
