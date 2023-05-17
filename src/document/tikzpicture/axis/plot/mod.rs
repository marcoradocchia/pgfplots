pub mod bidimensional;

use crate::{document::preamble::PgfPlotsLib, libs::statistics::histogram::Histogram};
use bidimensional::Plot2D;
use std::fmt;

// /// Implementors of this trait represent types who can be used inside an [`crate::Axis`]
// /// environment.
// ///
// /// In LaTeX this translates to the usage of the command `\addplot[...]{...}` like below:
// ///
// /// ```tex
// /// \begin{axis}[...]
// ///     \addplot[...]{...};
// ///     \draw [...] {...};
// /// \end{axis}
// /// ```
// pub trait AddPlot: fmt::Debug + fmt::Display + dyn_clone::DynClone {}
//
// // Allows `struct`s containing Box<dyn AddPlot> derive Clone.
// dyn_clone::clone_trait_object!(AddPlot);

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Plot {
    Draw(String),
    Plot2D(Plot2D),
    Histogram(Histogram),
}

impl From<Histogram> for Plot {
    fn from(histogram: Histogram) -> Self {
        Self::Histogram(histogram)
    }
}

impl From<Plot2D> for Plot {
    fn from(plot: Plot2D) -> Self {
        Self::Plot2D(plot)
    }
}

impl fmt::Display for Plot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Plot::Draw(draw) => write!(f, "\\draw {draw};"),
            Plot::Plot2D(plot) => write!(f, "{plot}"),
            Plot::Histogram(plot) => write!(f, "{plot}"),
        }
    }
}

impl Plot {
    /// Returns the required PGFPlots library for the [`Plot`].
    pub fn required_lib(&self) -> Option<PgfPlotsLib> {
        match self {
            Self::Draw(_) => None,
            Self::Plot2D(_) => None,
            Self::Histogram(_) => Some(PgfPlotsLib::Statistics),
        }
    }
}

#[cfg(test)]
mod test;
