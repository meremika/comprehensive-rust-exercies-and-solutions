#[derive(Debug)]
pub enum Language {
    Rust,
    Java,
    Perl,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Dependency {
    name: String,
    version_expression: String,
}

/// A representation of a software package.
#[derive(Debug)]
pub struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    dependencies: Vec<Dependency>,
    language: Option<Language>,
}

impl Package {
    /// Return a representation of this package as a dependency, for use in
    /// building other packages.
    pub fn as_dependency(&self) -> Dependency {
        Dependency {
            name: self.name.clone(),
            version_expression: self.version.clone(),
        }
    }
}

/// A builder for a Package. Use `build()` to create the `Package` itself.
pub struct PackageBuilder(Package);

impl PackageBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self(Package {
            name: name.into(),
            version: String::from(""),
            authors: vec![],
            dependencies: vec![],
            language: None,
        })
    }

    /// Set the package version.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }

    /// Set the package authors.
    pub fn authors(mut self, authors: Vec<String>) -> Self {
        self.0.authors = authors;
        self
    }

    /// Add an additional dependency.
    pub fn dependency(mut self, dependency: Dependency) -> Self {
        self.0.dependencies.push(dependency);
        self
    }

    /// Set the language. If not set, language defaults to None.
    pub fn language(mut self, language: Language) -> Self {
        self.0.language = Some(language);
        self
    }

    pub fn build(self) -> Package {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let base64 = PackageBuilder::new("base64").version("0.13").build();
        dbg!(&base64);
        let log = PackageBuilder::new("log")
            .version("0.4")
            .language(Language::Rust)
            .build();
        dbg!(&log);
        let serde = PackageBuilder::new("serde")
            .authors(vec!["djmitche".into()])
            .version(String::from("4.0"))
            .dependency(base64.as_dependency())
            .dependency(log.as_dependency())
            .build();
        dbg!(serde);
    }
}
