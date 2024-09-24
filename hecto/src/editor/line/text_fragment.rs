use super::grapheme_width::GraphemeWidth;
use super::ByteIdx;

#[derive(Clone, Debug)]
pub struct TextFragment {
    pub grapheme: String,
    pub rendered_width: GraphemeWidth,
    pub replacement: Option<char>,
    pub start: ByteIdx,
}
