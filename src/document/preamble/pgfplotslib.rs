use std::fmt;

/// PGFPlots library.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PgfPlotsLib {
    /// Custom library not yet implemented.
    /// This allows to specify pgfplots libraries not included in the enum.
    Custom(String),
    /// A library which generates small *popups* whenever one clicks into a plot.
    Clickable, // FIXME: PGFPlots documentation warns about some cases.
    /// A library which allows using dates as input coordinates,  together with support for hours
    /// and minutes.
    DatePlot,
    /// A library which allows to export Ti*k*Z pictures into a different `.pdf` (or `.eps`).
    External,
    /// A library which allows to fill the *area between* two arbitrary named plots.
    /// It can also identify segments of the intersections and fill the segments individually.
    FillBetween,
    /// A library which provides plot handlers for statistics
    /// (e.g. *hisograms*, *box-plots*, etc.).
    Statistics,
    /// A library which allows to use automatic typesetting of *units* in labels.
    Units,
    // TODO: follow unimplemented variants.
    // GroupPlots, FIXME: this requires the introduction of the `groupplot` environment.
    // PatchPlots,
    // Polar, FIXME: this requires the introduction of the `polaraxis` environment.
    // SmithChart, FIXME: this requires the introduction of the `smithchart` environment.
    // Ternary, FIXME: this requires the introduction of the `ternary` environment.
}

impl fmt::Display for PgfPlotsLib {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\\usepgfplotslibrary{{{}}}",
            match self {
                Self::Custom(lib) => lib,
                Self::Clickable => "clickable",
                Self::DatePlot => "dateplot",
                Self::External => "external",
                Self::FillBetween => "fillbetween",
                Self::Statistics => "statistics",
                Self::Units => "units",
            }
        )
    }
}

impl From<&str> for PgfPlotsLib {
    fn from(lib: &str) -> Self {
        Self::Custom(lib.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pgfplotslib() {
        assert_eq!("\\usepgfplotslibrary{{polar}}", PgfPlotsLib::from("polar").to_string());
        assert_eq!("\\usepgfplotslibrary{{units}}", PgfPlotsLib::Statistics.to_string());
        assert_eq!("\\usepgfplotslibrary{{statistics}}", PgfPlotsLib::Statistics.to_string());
    }
}
