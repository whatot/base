#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum AnnotationType {
    Match,
    SelectedMatch,
    Number,
    Keyword,
    Type,
    KnownValue,
    Char,
    LifetimeSpecifier,
    Comment,
}
