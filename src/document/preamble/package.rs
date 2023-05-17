use std::fmt;

/// LaTeX pacakges used to compile the standalone document.
///
/// ```text
/// \usepackage{name}[options]
/// ```
#[derive(Debug, Clone)]
pub struct Package {
    /// LaTeX package name.
    name: String,
    /// LaTeX package options.
    options: Vec<String>,
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\usepackage{{{}}}", self.name)?;
        if !self.options.is_empty() {
            write!(f, "[{}]", self.options.join(", "))?;
        }

        Ok(())
    }
}

impl<S> From<S> for Package
where
    S: Into<String>,
{
    fn from(name: S) -> Self {
        Self {
            name: name.into(),
            options: Vec::new(),
        }
    }
}

impl Package {
    /// Construct a new [`Package`].
    pub fn new(name: &str, options: &[&str]) -> Self {
        Self {
            name: name.to_string(),
            options: options.iter().map(|option| option.to_string()).collect(),
        }
    }

    /// Add an option to the LaTeX package import.
    pub fn add_option(&mut self, option: &str) {
        self.options.push(option.to_string());
    }
}
