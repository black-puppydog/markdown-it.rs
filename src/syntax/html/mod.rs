pub mod html_inline;
pub mod html_block;

use crate::MarkdownIt;

pub fn add(md: &mut MarkdownIt) {
    html_inline::add(md);
    html_block::add(md);
}
