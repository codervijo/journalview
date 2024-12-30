#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UiSection {
    Search,
    Logs,
    Selector,
    Help,
}

impl UiSection {
    // Get the next variant, returning None when reaching the end
    fn next(&self) -> UiSection {
        match self {
            UiSection::Search => UiSection::Logs,
            UiSection::Logs => UiSection::Selector,
            UiSection::Selector => UiSection::Search,
        }
    }
}