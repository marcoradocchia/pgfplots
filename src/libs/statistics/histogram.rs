use crate::document::tikzpicture::axis::plot::bidimensional::PlotOption;
use std::fmt;

// FIXME

/// PGFplots options passed to a hisotgram plot.
///
/// The most commonly used option-value pairs are variants of the [`HistogramOption`] enum.
/// The [`HistogramOption::Custom`] variant is provided to add unimplemented keys and
/// will be written verbatim in the options of the `\addplot[...]` command.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum HistogramOption {
    /// Custom option-value pairs that have not been implemented yet. These will be
    /// appended verbatim to the options of the `hist={...}` command.
    Custom(String),
    /// Tells [`Histogram`] how to get its data.
    /// FIXME, TODO
    /// If not specified, defaults to `y`.
    Data(()),
    /// Tells [`Histogram`] how to get its data, avoiding invocation of the math parser.
    /// In this case the value should be a numeric constant.
    /// FIXME, TODO
    DataValue(()),
    /// Allows to provide the min data range value manually.
    /// If not specified, defaults to `/pgfplots/xmin`.
    ///
    /// If empty, this value will be deduced from the input data range.
    DataMin(f64),
    /// Allows to provide the max data range value manually.
    /// If not specified, defaults to `/pgfplots/xmax`.
    ///
    /// If empty, this value will be deduced from the input data range.
    DataMax(f64),
    /// Number `N` of equally sized bins, with `(N + 1)` endpoints.
    /// If not specified, defaults to `10`.
    Bins(usize),
    /// Specifies the number of intervals to use.
    /// If not specified, defaults to `true`.
    ///
    /// If `true`, the [`Histogram`] will generate will generate N + 1 coordinates, with:
    /// ```text
    /// min = x_0 < x_1 < ... < x_N = max
    /// ```
    /// where `[min, max]` is the data range.
    /// In this case, the data points for `x_(N−1)` and `x_N` will get the same
    /// value, namely the number of elements in the last bin.
    /// This is (*only*) useful in conjunction with const plot or ybar interval.
    ///
    /// If `false`, the last data point will be omitted and exactly N coordinates will be
    /// generated. In this case, the right end point is not returned explicitly.
    Intervals(bool),
    /// Allows to compute a cumulative histogram.
    /// If not specified, defaults to `false`.
    ///
    /// A cumulative histogram uses the sum of all previous bins and the current one as final
    /// value.
    ///
    /// # Note
    /// Can be combined with [`HistogramOption::Density`].
    Cumulative(bool),
    /// Enables density estimation mode.
    /// If not specified, defaults to `false`.
    ///
    /// If active, the resulting data points will be
    /// *renormalized* such that the overall “mass” equals 1.
    ///
    /// # Note
    /// Can be combined with [`HistogramOption::Cumulative`].
    Density(bool),
    /// Allows to change the way the generated coordinates are visualized.
    /// If not specified, defaults to `ybar interval`.
    ///
    /// `hist/handler` is a style, so it translates to `hist/handler/.style={...}`.
    Handler(String),
    // TODO:
    // DataFilter(()),
    // DataCoordTrafo(()),
    // DataCoordInvTrafo(()),
    // SymbolicCoords(),
}

impl fmt::Display for HistogramOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HistogramOption::Custom(value) => write!(f, "{value}"),
            HistogramOption::Data(_) => todo!(),
            HistogramOption::DataValue(_) => todo!(),
            HistogramOption::DataMin(value) => write!(f, "data min={{{value}}}"),
            HistogramOption::DataMax(value) => write!(f, "data max={{{value}}}"),
            HistogramOption::Bins(n) => write!(f, "bins={n}"),
            HistogramOption::Intervals(value) => write!(f, "intervals={value}"),
            HistogramOption::Cumulative(value) => write!(f, "cumulative={value}"),
            HistogramOption::Density(value) => write!(f, "density={value}"),
            HistogramOption::Handler(value) => write!(f, "handler/.style={{{value}}}"),
        }
    }
}

/// Histogram plot inside an [`crate::document::tikzpicture::axis::Axis`].
/// Implies the import of the pgfplots library `statistics`:
/// ```text
/// \usepgfplotslibrary{statistics}
/// ```
///
/// Adding a [`Histogram`] to an [`crate::document::tikzpicture::axis::Axis`] environment
/// is equivalent to:
///
/// ```text
/// \addplot[hist={...}, ...]
/// ```
///
/// # Examples
///
/// ```no_run
/// TODO
/// ```
#[derive(Clone, Debug, Default)]
pub struct Histogram {
    /// Plot options.
    options: Vec<PlotOption>,
    /// Histogram specific options.
    hist_options: Vec<HistogramOption>,
    /// Histogram data.
    pub data: Vec<f64>, // TODO: what if one wants to pass data in a file?
}

impl<D> From<D> for Histogram
where
    D: Into<Vec<f64>>,
{
    fn from(data: D) -> Self {
        Self {
            options: vec![],
            hist_options: vec![],
            data: data.into(),
        }
    }
}

impl Histogram {
    /// Constructs a new, empty histogram plot.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pgfplots::libs::statistics::histogram::Histogram;
    ///
    /// let histogram = Histogram::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Add an option to control the appearance of the histogram plot. This will overwrite
    /// any previous mutually exclusive key.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pgfplots::libs::statistics::histogram::{Histogram, HistogramOption};
    ///
    /// let histogram = Histogram::new();
    /// // TODO
    /// ```
    pub fn option(mut self, option: PlotOption) -> Self {
        match option {
            PlotOption::Custom(_) => (),
            _ => {
                if let Some(index) = self
                    .options
                    .iter()
                    .position(|opt| std::mem::discriminant(opt) == std::mem::discriminant(opt))
                {
                    self.options.remove(index);
                }
            }
        }
        self.options.push(option);
        self
    }

    /// Add an option to control the appearance of the histogram plot. This will overwrite
    /// any previous mutually exclusive key.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pgfplots::libs::statistics::histogram::{Histogram, HistogramOption};
    ///
    /// let histogram = Histogram::new()
    ///     .hist_option(HistogramOption::Bins(bins));
    /// ```
    pub fn hist_option(mut self, option: HistogramOption) -> Self {
        match option {
            HistogramOption::Custom(_) => (),
            _ => {
                if let Some(index) = self
                    .options
                    .iter()
                    .position(|opt| std::mem::discriminant(opt) == std::mem::discriminant(opt))
                {
                    self.options.remove(index);
                }
            }
        }
        self.hist_options.push(option);
        self
    }

    /// Sets the number `num` of equally sized bins.
    ///
    /// Convenience method for:
    /// ```no_run
    /// let histogram = Histogram::new()
    ///     .hist_option(HistogramOption::Bins(bins));
    /// ```
    pub fn bins(mut self, bins: usize) -> Self {
        self.add_hist_option(HistogramOption::Bins(bins));
        self
    }

    /// Sets the max data range manually.
    ///
    /// Convenience method for:
    /// ```no_run
    /// let histogram = Histogram::new()
    ///     .hist_option(HistogramOption::DataMax(max));
    /// ```
    pub fn data_max(mut self, max: f64) -> Self {
        self.add_hist_option(HistogramOption::DataMax(max));
        self
    }

    /// Sets the max data range manually.
    ///
    /// Convenience method for:
    /// ```no_run
    /// let histogram = Histogram::new()
    ///     .hist_option(HistogramOption::DataMin(min));
    /// ```
    pub fn data_min(mut self, min: f64) -> Self {
        self.add_hist_option(HistogramOption::DataMin(min));
        self
    }

    /// Enables histogram normalization.
    ///
    /// Convenience method for:
    /// ```no_run
    /// let histogram = Histogram::new()
    ///     .hist_option(HistogramOption::Density(true));
    /// ```
    pub fn normalize(mut self) -> Self {
        self.add_hist_option(HistogramOption::Density(true));
        self
    }

    /// Add an option to control the appearance of the histogram plot. This will overwrite
    /// any previous mutually exclusive key.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pgfplots::libs::statistics::histogram::{Histogram, HistogramOption};
    ///
    /// let mut histogram = Histogram::new();
    /// // TODO
    /// ```
    pub fn add_option(&mut self, option: PlotOption) {
        match option {
            PlotOption::Custom(_) => (),
            _ => {
                if let Some(index) = self
                    .options
                    .iter()
                    .position(|opt| std::mem::discriminant(opt) == std::mem::discriminant(opt))
                {
                    self.options.remove(index);
                }
            }
        }
        self.options.push(option);
    }

    /// Add an option to control the appearance of the histogram plot. This will overwrite
    /// any previous mutually exclusive key.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pgfplots::libs::statistics::histogram::{Histogram, HistogramOption};
    ///
    /// let mut histogram = Histogram::new();
    /// histogram.add_hist_option(HistogramOption::Bins(3));
    /// ```
    pub fn add_hist_option(&mut self, option: HistogramOption) {
        match option {
            HistogramOption::Custom(_) => (),
            _ => {
                if let Some(index) = self
                    .options
                    .iter()
                    .position(|opt| std::mem::discriminant(opt) == std::mem::discriminant(opt))
                {
                    self.options.remove(index);
                }
            }
        }
        self.hist_options.push(option);
    }

    /// Sets the number `num` of equally sized bins.
    ///
    /// Convenience method for:
    /// ```no_run
    /// let mut histogram = Histogram::new();
    /// hisotgram.add_hist_option(HistogramOption::Bins(value));
    /// ```
    pub fn set_bins(&mut self, bins: usize) {
        self.add_hist_option(HistogramOption::Bins(bins));
    }

    /// Sets the max data range manually.
    ///
    /// Convenience method for:
    /// ```no_run
    /// let mut histogram = Histogram::new();
    /// hisotgram.add_hist_option(HistogramOption::DataMax(value));
    /// ```
    pub fn set_data_max(&mut self, max: f64) {
        self.add_hist_option(HistogramOption::DataMax(max));
    }

    /// Sets the max data range manually.
    ///
    /// Convenience method for:
    /// ```no_run
    /// let mut histogram = Histogram::new();
    /// hisotgram.add_option(HistogramOption::DataMin(value));
    /// ```
    pub fn set_data_min(&mut self, min: f64) {
        self.add_hist_option(HistogramOption::DataMin(min));
    }

    /// Enables histogram normalization.
    ///
    /// Convenience method for:
    /// ```no_run
    /// let mut histogram = Histogram::new();
    /// hisotgram.add_option(HistogramOption::Density(true));
    /// ```
    pub fn set_normalize(&mut self) {
        self.add_hist_option(HistogramOption::Density(true));
    }
}

impl fmt::Display for Histogram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\t\\addplot+ [")?;

        if !self.hist_options.is_empty() {
            writeln!(f, "\n\t\thist={{")?;
            for hist_opt in self.hist_options.iter() {
                writeln!(f, "\t\t\t{hist_opt},")?;
            }
            write!(f, "\t\t}}")?;
        } else {
            write!(f, "\n\t\thist,")?;
        }

        writeln!(f)?;

        if !self.options.is_empty() {
            for opt in self.options.iter() {
                writeln!(f, "\t\t{opt},")?;
            }
        }

        writeln!(f, "\t] table [row sep=\\\\, y index=0] {{\n\t\tdata \\\\")?; // TODO: here maybe pass table options

        for datum in self.data.iter() {
            writeln!(f, "\t\t{datum} \\\\")?;
        }

        write!(f, "\t}};")?;

        Ok(())
    }
}
