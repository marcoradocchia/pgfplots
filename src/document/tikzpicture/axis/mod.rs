pub mod plot;

use crate::{
    document::{
        preamble::PgfPlotsLib,
        tikzpicture::axis::plot::{bidimensional::Plot2D, Plot},
    },
    libs::statistics::histogram::Histogram,
};
use itertools::Itertools;
use std::fmt;

/// PGFPlots options passed to the [`Axis`] environment.
///
/// The most commonly used key-value pairs are variants of the [`AxisOption`] enum.
/// The [`AxisOption::Custom`] variant is provided to add unimplemented keys and
/// will be written verbatim in the options of the [`Axis`] environment.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum AxisOption {
    /// Custom key-value pairs that have not been implemented. These will be
    /// appended verbatim to the options of the [`Axis`].
    Custom(String),
    /// Control the `x` axis minimum limit.
    XMin(f64),
    /// Control the `x` axis maximum limit.
    XMax(f64),
    /// Control the `y` axis minimum limit.
    YMin(f64),
    /// Control the `y` axis maximum limit.
    YMax(f64),
    /// Control the `z` axis minimum limit.
    ZMin(f64),
    /// Control the `z` axis maximum limit.
    ZMax(f64),
    /// Control the `x`,`y`,`z` axis minimum limit.
    Min(f64),
    /// Control the `x`,`y`,`z` axis maximum limit.
    Max(f64),
    /// Control the scaling of the `x` axis.
    XMode(Scale),
    /// Control the scaling of the `y` axis.
    YMode(Scale),
    /// Control the title of the axis environment.
    Title(String),
    /// Control the label of the `x` axis.
    XLabel(String),
    /// Control the label of the `y` axis.
    YLabel(String),
    /// Control the `x` ticks manually (`xtick` option).
    XTick(Ticks),
    /// Control the `y` ticks manually (`ytick` option).
    YTick(Ticks),
    /// Control the label of the `x` axis ticks.
    XTickLabel(String),
    /// Control the label of the `y` axis ticks.
    YTickLabel(String),
    /// Control the tick labels of the `x` axis.
    XTickLabels(TickLabels),
    /// Control the tick labels of the `y` axis.
    YTickLabels(TickLabels),
    /// Control the tick labels of the `z` axis.
    ZTickLabels(TickLabels),
    /// Control the axis `x` line type.
    AxisXLine(AxisXLine),
    /// Control the axis `x` line type, without correcting the positions of axis labels,
    /// tick lines or other keys which are (possibly) affected by a changed axis line.
    AxisXLineAst(AxisXLine),
    /// Control the axis `y` line type.
    AxisYLine(AxisYLine),
    /// Control the axis `y` line type, without correcting the positions of axis labels,
    /// tick lines or other keys which are (possibly) affected by a changed axis line.
    AxisYLineAst(AxisYLine),
    /// Control the axis `z` line type.
    AxisZLine(AxisZLine),
    /// Control the axis `z` line type, without correcting the positions of axis labels,
    /// tick lines or other keys which are (possibly) affected by a changed axis line.
    AxisZLineAst(AxisZLine),
    /// Control the axes line type.
    AxisLines(AxisLines),
    /// Control the axes line type, without correcting the positions of axis labels,
    /// tick lines or other keys which are (possibly) affected by a changed axis line.
    AxisLinesAst(AxisLines),
    /// Control the axis grid lines.
    Grid(Grid),
    // /// Control the legend style.
    // LegendStyle(String),
}

impl From<&str> for AxisOption {
    fn from(option: &str) -> Self {
        Self::Custom(option.to_string())
    }
}

impl fmt::Display for AxisOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AxisOption::Custom(option) => write!(f, "{option}"),
            AxisOption::XMin(value) => write!(f, "xmin={{{value}}}"),
            AxisOption::XMax(value) => write!(f, "xmax={{{value}}}"),
            AxisOption::YMin(value) => write!(f, "ymin={{{value}}}"),
            AxisOption::YMax(value) => write!(f, "ymax={{{value}}}"),
            AxisOption::ZMin(value) => write!(f, "zmin={{{value}}}"),
            AxisOption::ZMax(value) => write!(f, "zmax={{{value}}}"),
            AxisOption::Min(value) => write!(f, "min={{{value}}}"),
            AxisOption::Max(value) => write!(f, "max={{{value}}}"),
            AxisOption::XMode(value) => write!(f, "xmode={value}"),
            AxisOption::YMode(value) => write!(f, "ymode={value}"),
            AxisOption::Title(value) => write!(f, "title={{{value}}}"),
            AxisOption::XTick(value) => write!(f, "xtick={{{value}}}"),
            AxisOption::YTick(value) => write!(f, "ytick={{{value}}}"),
            AxisOption::XLabel(value) => write!(f, "xlabel={{{value}}}"),
            AxisOption::YLabel(value) => write!(f, "ylabel={{{value}}}"),
            AxisOption::XTickLabel(value) => write!(f, "xticklabel={{{value}}}"),
            AxisOption::YTickLabel(value) => write!(f, "yticklabel={{{value}}}"),
            AxisOption::XTickLabels(value) => write!(f, "xticklabels={{{value}}}"),
            AxisOption::YTickLabels(value) => write!(f, "yticklabels={{{value}}}"),
            AxisOption::ZTickLabels(value) => write!(f, "zticklabels={{{value}}}"),
            AxisOption::AxisXLine(value) => write!(f, "axis x line={value}"),
            AxisOption::AxisXLineAst(value) => write!(f, "axis x line*={value}"),
            AxisOption::AxisYLine(value) => write!(f, "axis y line={value}"),
            AxisOption::AxisYLineAst(value) => write!(f, "axis y line*={value}"),
            AxisOption::AxisZLine(value) => write!(f, "axis z line={value}"),
            AxisOption::AxisZLineAst(value) => write!(f, "axis z line*={value}"),
            AxisOption::AxisLines(value) => write!(f, "axis lines={value}"),
            AxisOption::AxisLinesAst(value) => write!(f, "axis lines*={value}"),
            AxisOption::Grid(value) => write!(f, "grid={value}"),
        }
    }
}

/// Axis environment inside a [`crate::Picture`].
///
/// An [`Axis`] is equivalent to the PGFPlots axis environment:
///
/// ```text
/// \begin{axis}[AxisKeys]
///     % plots
/// \end{axis}
/// ```
///
/// # Examples
///
/// ```no_run
/// use pgfplots::{
///     error::ShowPdfError,
///     engine::LatexEngine,
///     document::tikzpicture::{axis::Axis, TikzPicture},
/// };
/// fn main() -> Result<(), ShowPdfError> {
///     let mut axis = Axis::new();
///     axis.set_title("Picture of $\\gamma$ rays");
///     axis.set_x_label("$x$~[m]");
///     axis.set_y_label("$y$~[m]");
///
///     TikzPicture::from(axis).show_pdf(LatexEngine::LuaLatex)?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Default, Clone)]
pub struct Axis {
    options: Vec<AxisOption>,
    plots: Vec<Plot>,
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\begin{{axis}}")?;
        // If there are keys, print one per line. It makes it easier for a
        // human to find individual keys later.
        if !self.options.is_empty() {
            writeln!(f, "[")?;
            for key in self.options.iter() {
                writeln!(f, "\t{key},")?;
            }
            write!(f, "]")?;
        }
        writeln!(f)?;

        for plot in self.plots.iter() {
            writeln!(f, "{plot}")?;
        }

        write!(f, "\\end{{axis}}")?;

        Ok(())
    }
}

impl From<Plot> for Axis {
    fn from(plot: Plot) -> Self {
        Axis {
            options: vec![],
            plots: vec![plot],
        }
    }
}

impl From<Plot2D> for Axis {
    fn from(plot: Plot2D) -> Self {
        Self::from(Plot::from(plot))
    }
}

impl From<Histogram> for Axis {
    fn from(histogram: Histogram) -> Self {
        Self::from(Plot::from(histogram))
    }
}

impl Axis {
    /// Creates a new, empty axis environment.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let axis = Axis::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the `x` axis minimum limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new()
    ///     .x_min(0.0);
    /// ```
    pub fn x_min(self, min: f64) -> Self {
        self.option(AxisOption::XMin(min))
    }

    /// Set the `x` axis maximum limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new()
    ///     .x_max(10.0);
    /// ```
    pub fn x_max(self, max: f64) -> Self {
        self.option(AxisOption::XMax(max))
    }

    /// Set the `y` axis minimum limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new()
    ///     .y_min(0.0);
    /// ```
    pub fn y_min(self, min: f64) -> Self {
        self.option(AxisOption::YMin(min))
    }

    /// Set the `y` axis maximum limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new()
    ///     .y_max(10.0);
    /// ```
    pub fn y_max(self, max: f64) -> Self {
        self.option(AxisOption::YMax(max))
    }

    /// Set the `z` axis minimum limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new()
    ///     .z_min(0.0);
    /// ```
    pub fn z_min(self, min: f64) -> Self {
        self.option(AxisOption::ZMin(min))
    }

    /// Set the `z` axis maximum limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new()
    ///     .z_max(10.0);
    /// ```
    pub fn z_max(self, max: f64) -> Self {
        self.option(AxisOption::ZMax(max))
    }

    /// Set the `x`,`y`,`z`, axis minimum limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new()
    ///     .min(0.0);
    /// ```
    pub fn min(self, min: f64) -> Self {
        self.option(AxisOption::Min(min))
    }

    /// Set the `x`,`y`,`z`, axis maximum limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new()
    ///     .max(10.0);
    /// ```
    pub fn max(self, max: f64) -> Self {
        self.option(AxisOption::Max(max))
    }

    /// Set the title of the axis environment. This can be valid LaTeX e.g. inline math.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new()
    ///     .title("My plot: $y = x^2$");
    /// ```
    pub fn title<S>(self, title: S) -> Self
    where
        S: Into<String>,
    {
        self.option(AxisOption::Title(title.into()))
    }

    /// Set the label of the `x` axis. This can be valid LaTeX e.g. inline math.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new()
    ///     .x_label("$x$~[m]");
    /// ```
    pub fn x_label<S>(self, label: S) -> Self
    where
        S: Into<String>,
    {
        self.option(AxisOption::XLabel(label.into()))
    }

    /// Set the label of the `y` axis. This can be valid LaTeX e.g. inline math.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new()
    ///     .y_label("$y$~[m]");
    /// ```
    pub fn y_label<S>(self, label: S) -> Self
    where
        S: Into<String>,
    {
        self.option(AxisOption::YLabel(label.into()))
    }

    /// Set the `x` axis ticks.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new()
    ///     .x_ticks([1.2, 3.0, 4.4]);
    pub fn x_ticks<T>(self, ticks: T) -> Self
    where
        T: Into<Ticks>,
    {
        self.option(AxisOption::XTick(ticks.into()))
    }

    /// Set the `y` axis ticks.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new()
    ///     .y_ticks([1.2, 3.0, 4.4]);
    pub fn y_ticks<T>(self, ticks: T) -> Self
    where
        T: Into<Ticks>,
    {
        self.option(AxisOption::YTick(ticks.into()))
    }

    /// Set the `x` axis tick labels.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new()
    ///     .x_ticks([3.14, 6.28])
    ///     .x_tick_labels([r#"$\pi$"#, r#"$2 \pi$"#]);
    pub fn x_tick_labels<L>(self, tick_labels: L) -> Self
    where
        L: Into<TickLabels>,
    {
        self.option(AxisOption::XTickLabels(tick_labels.into()))
    }

    /// Set the `y` axis tick labels.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new()
    ///     .y_ticks([3.14, 6.28])
    ///     .y_tick_labels([r#"$\pi$"#, r#"$2 \pi$"#]);
    pub fn y_tick_labels<L>(self, tick_labels: L) -> Self
    where
        L: Into<TickLabels>,
    {
        self.option(AxisOption::YTickLabels(tick_labels.into()))
    }

    /// Set the `z` axis tick labels.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new()
    ///     .z_ticks([3.14, 6.28])
    ///     .z_tick_labels([r#"$\pi$"#, r#"$2 \pi$"#]);
    pub fn z_tick_labels<L>(self, tick_labels: L) -> Self
    where
        L: Into<TickLabels>,
    {
        self.option(AxisOption::ZTickLabels(tick_labels.into()))
    }

    /// Add a option to control the appearance of the axis. This will overwrite
    /// any previous mutually exclusive option.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::{Axis, AxisOption, Scale};
    ///
    /// let mut axis = Axis::new()
    ///     .option(AxisKey::YMode(Scale::Log));
    /// ```
    pub fn option(mut self, option: AxisOption) -> Self {
        match option {
            AxisOption::Custom(_) => (),
            _ => {
                if let Some(index) = self
                    .options
                    .iter()
                    .position(|idx| std::mem::discriminant(idx) == std::mem::discriminant(&option))
                {
                    self.options.remove(index);
                }
            }
        }
        self.options.push(option);
        self
    }

    /// Add a [`Plot`] to the [`Axis`].
    pub fn plot(mut self, plot: Plot) -> Self {
        self.plots.push(plot);
        self
    }

    /// Set the `x` axis minimum limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_x_min(0.0);
    /// ```
    pub fn set_x_min(&mut self, min: f64) {
        self.add_option(AxisOption::XMin(min));
    }

    /// Set the `x` axis maximum limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_x_max(10.0);
    /// ```
    pub fn set_x_max(&mut self, max: f64) {
        self.add_option(AxisOption::XMax(max));
    }

    /// Set the `y` axis minimum limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_y_min(0.0);
    /// ```
    pub fn set_y_min(&mut self, min: f64) {
        self.add_option(AxisOption::YMin(min));
    }

    /// Set the `y` axis maximum limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_y_max(10.0);
    /// ```
    pub fn set_y_max(&mut self, max: f64) {
        self.add_option(AxisOption::YMax(max));
    }

    /// Set the `z` axis minimum limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_z_min(0.0);
    /// ```
    pub fn set_z_min(&mut self, min: f64) {
        self.add_option(AxisOption::ZMin(min));
    }

    /// Set the `z` axis maximum limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_z_max(10.0);
    /// ```
    pub fn set_z_max(&mut self, max: f64) {
        self.add_option(AxisOption::ZMax(max));
    }

    /// Set the `x`,`y`,`z`, axis minimum limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_min(0.0);
    /// ```
    pub fn set_min(&mut self, min: f64) {
        self.add_option(AxisOption::Min(min));
    }

    /// Set the `x`,`y`,`z`, axis maximum limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_max(10.0);
    /// ```
    pub fn set_max(&mut self, max: f64) {
        self.add_option(AxisOption::Max(max));
    }

    /// Set the title of the axis environment. This can be valid LaTeX e.g. inline math.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_title("My plot: $y = x^2$");
    /// ```
    pub fn set_title<S>(&mut self, title: S)
    where
        S: Into<String>,
    {
        self.add_option(AxisOption::Title(title.into()));
    }

    /// Set the label of the `x` axis. This can be valid LaTeX e.g. inline math.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_x_label("$x$~[m]");
    /// ```
    pub fn set_x_label<S>(&mut self, label: S)
    where
        S: Into<String>,
    {
        self.add_option(AxisOption::XLabel(label.into()));
    }

    /// Set the label of the `y` axis. This can be valid LaTeX e.g. inline math.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_y_label("$y$~[m]");
    /// ```
    pub fn set_y_label<S>(&mut self, label: S)
    where
        S: Into<String>,
    {
        self.add_option(AxisOption::YLabel(label.into()));
    }

    /// Set the `x` axis ticks.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_x_ticks([1.2, 3.0, 4.4]);
    pub fn set_x_ticks<T>(&mut self, ticks: T)
    where
        T: Into<Ticks>,
    {
        self.add_option(AxisOption::XTick(ticks.into()));
    }

    /// Set the `y` axis ticks.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_y_ticks([1.2, 3.0, 4.4]);
    pub fn set_y_ticks<T>(&mut self, ticks: T)
    where
        T: Into<Ticks>,
    {
        self.add_option(AxisOption::YTick(ticks.into()));
    }

    /// Set the `x` axis tick labels.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_x_ticks([3.14, 6.28]);
    /// axis.set_x_tick_labels([r#"$\pi$"#, r#"$2 \pi$"#]);
    pub fn set_x_tick_labels<L>(&mut self, tick_labels: L)
    where
        L: Into<TickLabels>,
    {
        self.add_option(AxisOption::XTickLabels(tick_labels.into()));
    }

    /// Set the `y` axis tick labels.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_y_ticks([3.14, 6.28]);
    /// axis.set_y_tick_labels([r#"$\pi$"#, r#"$2 \pi$"#]);
    pub fn set_y_tick_labels<L>(&mut self, tick_labels: L)
    where
        L: Into<TickLabels>,
    {
        self.add_option(AxisOption::YTickLabels(tick_labels.into()));
    }

    /// Set the `z` axis tick labels.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_z_ticks([3.14, 6.28]);
    /// axis.set_z_tick_labels([r#"$\pi$"#, r#"$2 \pi$"#]);
    pub fn set_z_tick_labels<L>(&mut self, tick_labels: L)
    where
        L: Into<TickLabels>,
    {
        self.add_option(AxisOption::ZTickLabels(tick_labels.into()));
    }

    /// Add a option to control the appearance of the axis. This will overwrite
    /// any previous mutually exclusive option.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::{Axis, AxisOption, Scale};
    ///
    /// let mut axis = Axis::new();
    /// axis.add_option(AxisOption::YMode(Scale::Log));
    /// ```
    pub fn add_option(&mut self, option: AxisOption) {
        match option {
            AxisOption::Custom(_) => (),
            _ => {
                if let Some(index) = self
                    .options
                    .iter()
                    .position(|idx| std::mem::discriminant(idx) == std::mem::discriminant(&option))
                {
                    self.options.remove(index);
                }
            }
        }
        self.options.push(option);
    }

    /// Add a [`Plot`] to the [`Axis`].
    pub fn add_plot(&mut self, plot: Plot) {
        self.plots.push(plot);
    }

    /// Returns a vector of [`PgfPlotsLib`]s required by the contained plots.
    pub fn required_libs(&self) -> Vec<PgfPlotsLib> {
        self.plots
            .iter()
            .filter_map(|plot| plot.required_lib())
            .collect()
    }
}

/// Control the scaling of an axis.
#[derive(Clone, Copy, Debug)]
pub enum Scale {
    /// Logarithmic scaling i.e. apply the natural logarithm to each coordinate.
    Log,
    /// Linear scaling of the coordinates.
    Normal,
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scale::Log => write!(f, "log"),
            Scale::Normal => write!(f, "normal"),
        }
    }
}

/// Control the appearance of the `x` axis.
#[derive(Debug, Default, Clone, Copy)]
pub enum AxisXLine {
    #[default]
    Box,
    Top,
    Middle,
    Center,
    Bottom,
    None,
}

impl fmt::Display for AxisXLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AxisXLine::Box => "box",
                AxisXLine::Top => "top",
                AxisXLine::Middle => "middle",
                AxisXLine::Center => "center",
                AxisXLine::Bottom => "bottom",
                AxisXLine::None => "none",
            }
        )
    }
}

/// Control the appearance of every axis.
#[derive(Debug, Default, Clone, Copy)]
pub enum AxisLines {
    #[default]
    Box,
    Left,
    Middle,
    Center,
    Right,
    None,
}

impl fmt::Display for AxisLines {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AxisYLine::Box => "box",
                AxisYLine::Left => "left",
                AxisYLine::Middle => "middle",
                AxisYLine::Center => "center",
                AxisYLine::Right => "right",
                AxisYLine::None => "none",
            }
        )
    }
}

/// Control the appearance of the `z` axis.
pub type AxisZLine = AxisLines;

/// Control the appearance of the `y` axis.
pub type AxisYLine = AxisLines;

/// Control the grid lines.
/// # Note
/// Major grid lines are placed at the normal tick positions (see `xmajorticks`)
/// while minor grid lines are placed at minor ticks (see `xminorticks`).
#[derive(Debug, Clone)]
pub enum Grid {
    /// Placed at the *normal tick positions*.
    Major,
    /// Placed at the *minor tick positions*.
    Minor,
    /// Placed at both the *normal* and *minor tick positions*.
    Both,
    /// Disabled grid lines.
    None,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Grid::Major => write!(f, "major"),
            Grid::Minor => write!(f, "minor"),
            Grid::Both => write!(f, "both"),
            Grid::None => write!(f, "none"),
        }
    }
}

/// Control the axis ticks by assigning a list of positions where ticks shall be placed.
#[derive(Debug, Clone)]
pub struct Ticks(Vec<f64>);

impl<T> From<T> for Ticks
where
    T: Into<Vec<f64>>,
{
    fn from(ticks: T) -> Self {
        Self(ticks.into())
    }
}

impl fmt::Display for Ticks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.iter().join(", "))
    }
}

/// Control the axis tick labels by assigning a list of tick labels to each tick position
#[derive(Debug, Clone)]
pub struct TickLabels(Vec<String>);

impl<L> From<L> for TickLabels
where
    L: Into<Vec<String>>,
{
    fn from(tick_labels: L) -> Self {
        Self(tick_labels.into())
    }
}

impl fmt::Display for TickLabels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.iter().join(", "))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ticks() {
        let ticks = Ticks::from([1.0, 2.2, 3.3, 4.0].as_slice());
        assert_eq!(r#"1, 2.2, 3.3, 4"#, ticks.to_string());
    }
}
