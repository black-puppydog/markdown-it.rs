pub mod inline;
pub mod core;

use crate::MarkdownIt;

pub fn add(md: &mut MarkdownIt) {
    inline::text::add(md);
    inline::balance_pairs::add(md);
    inline::fragments_join::add(md);

    core::normalize::add(md);
    core::block::add(md);
    core::inline::add(md);
    core::text_join::add(md);
}
