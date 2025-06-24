pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: &str);
}

pub struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

pub struct Filter<L, F> {
    inner: L,
    filter: F,
}

impl<L, F> Filter<L, F>
where
    L: Logger,
    F: Fn(u8, &str) -> bool,
{
    pub fn new(inner: L, filter: F) -> Self {
        Self { inner, filter }
    }
}

impl<L, F> Logger for Filter<L, F>
where
    L: Logger,
    F: Fn(u8, &str) -> bool,
{
    fn log(&self, verbosity: u8, message: &str) {
        if (self.filter)(verbosity, message) {
            self.inner.log(verbosity, message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
        logger.log(5, "FYI");
        logger.log(1, "yikes, something went wrong");
        logger.log(2, "uhoh");
    }
}
