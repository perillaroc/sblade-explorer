pub mod analyze;
pub mod catalog;
/// Synthetic saves shared by the test suites; not used at runtime.
pub mod fixture;
pub mod gvas;
pub mod i18n;
mod paths;
pub mod report;
pub mod savegame;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::VERSION;

    #[test]
    fn version_is_not_empty() {
        assert!(!VERSION.is_empty());
    }
}
