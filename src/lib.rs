#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
//! A Rust library to generate publication-quality figures.
//!
//! This crate is a PGFPlots code generator, and provides utilities to create,
//! customize, and compile high-quality plots. The `tectonic` feature allows
//! users to fully process figures without relying on any externally installed
//! software.
//!
//! The library's API is designed to feel natural for LaTeX and PGFPlots users,
//! but no previous experience is required to start generating
//! publication-quality plots in Rust.
//!
//! # Quick Start
//!
//! To get you started quickly, the easiest way to generate a plot is to use a
//! [`Plot2D`]. Plotting a quadratic function is as simple as:
//!
//! ```no_run
//! # use pgfplots::ShowPdfError;
//! # fn main() -> Result<(), ShowPdfError> {
//! use pgfplots::{axis::plot::Plot2D, Engine, Picture};
//!
//! let mut plot = Plot2D::new();
//! plot.coordinates = (-100..100)
//!     .into_iter()
//!     .map(|i| (f64::from(i), f64::from(i*i)).into())
//!     .collect();
//!
//! // The `Engine::PdfLatex` variant requires a working LaTeX installation with
//! // the `pgfplots` package installed.
//! // The `Engine::Tectonic` variant (enabled by the `tectonic` feature) allows
//! // users to fully process figures without relying on any externally
//! // installed software.
//! Picture::from(plot).show_pdf(Engine::PdfLatex)?;
//! # Ok(())
//! # }
//! ```
//!
//! It is possible to show multiple plots in the same axis environment by
//! creating an [`Axis`] and adding plots to it. An [`Axis`] and its individual
//! [`Plot2D`]s are customized by [`AxisKey`]s and [`PlotKey`]s respectively.

// TODO: add extenralization

pub mod document;
pub mod error;
pub mod engine;
pub mod libs;
pub mod output;

pub type Result<T> = std::result::Result<T, error::PgfPlotsError>;

#[cfg(test)]
mod tests {}
