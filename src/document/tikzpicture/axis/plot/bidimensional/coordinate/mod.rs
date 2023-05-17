use std::fmt;

/// Coordinate in a two-dimensional plot.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Coordinate2D {
    pub x: f64,
    pub y: f64,
    /// By default, error bars are not drawn (even if it is a [`Some`]). These
    /// are only drawn if both [`PlotKey::XError`] and
    /// [`crate::document::tikzpicture::axis::plot::bidimensional::PlotKey::XErrorDirection`]
    /// are set in the [`crate::document::tikzpicture::axis::plot::bidimensional::Plot2D`].
    pub error_x: Option<f64>,
    /// By default, error bars are not drawn (even if it is a [`Some`]). These
    /// are only drawn if both
    /// [`crate::document::tikzpicture::axis::plot::bidimensional::PlotKey::YError`] and
    /// [`crate::document::tikzpicture::axis::plot::bidimensional::PlotKey::YErrorDirection`]
    /// are set in the [`crate::document::tikzpicture::axis::plot::bidimensional::Plot2D`].
    pub error_y: Option<f64>,
    // What to do when `point meta=explicit` in plot?
    // Should we add an Option<point_meta> here?
    // Is `point meta` skipped same as error when it is not set?
}

impl fmt::Display for Coordinate2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)?;

        if self.error_x.is_some() || self.error_y.is_some() {
            let error_x = self.error_x.unwrap_or(0.0);
            let error_y = self.error_y.unwrap_or(0.0);
            write!(f, "\t+- ({error_x},{error_y})")?;
        }

        Ok(())
    }
}

impl From<(f64, f64)> for Coordinate2D {
    /// Conversion from an `(x,y)` tuple into a two-dimensional coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::plot::bidimensional::coordinate::Coordinate2D;
    ///
    /// let point: Coordinate2D = (1.0, -1.0).into();
    ///
    /// assert_eq!(point.x, 1.0);
    /// assert_eq!(point.y, -1.0);
    /// assert!(point.error_x.is_none());
    /// assert!(point.error_y.is_none());
    /// ```
    fn from(coordinate: (f64, f64)) -> Self {
        Coordinate2D {
            x: coordinate.0,
            y: coordinate.1,
            error_x: None,
            error_y: None,
        }
    }
}

impl From<&(f64, f64)> for Coordinate2D {
    fn from(coordinate: &(f64, f64)) -> Self {
        Self::from(*coordinate)
    }
}

impl From<(f64, f64, Option<f64>, Option<f64>)> for Coordinate2D {
    /// Conversion from an `(x,y,error_x,error_y)` tuple into a two-dimensional
    /// coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::document::tikzpicture::axis::plot::bidimensional::coordinate::Coordinate2D;
    ///
    /// let point: Coordinate2D = (1.0, -1.0, None, Some(3.0)).into();
    ///
    /// assert_eq!(point.x, 1.0);
    /// assert_eq!(point.y, -1.0);
    /// assert!(point.error_x.is_none());
    /// assert_eq!(point.error_y.unwrap(), 3.0);
    /// ```
    fn from(coordinate: (f64, f64, Option<f64>, Option<f64>)) -> Self {
        Coordinate2D {
            x: coordinate.0,
            y: coordinate.1,
            error_x: coordinate.2,
            error_y: coordinate.3,
        }
    }
}

impl From<&(f64, f64, Option<f64>, Option<f64>)> for Coordinate2D {
    fn from(coordinate: &(f64, f64, Option<f64>, Option<f64>)) -> Self {
        Self::from(*coordinate)
    }
}

#[cfg(test)]
mod test {}
