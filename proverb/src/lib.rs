pub fn build_proverb(list: &[&str]) -> String {
    // unimplemented!("build a proverb from this list of items: {:?}", list)
    match list.first() {
        None => String::new(),
        Some(word) => list
            .windows(2)
            .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
            .chain(std::iter::once(format!(
                "And all for the want of a {}",
                word
            )))
            .collect(),
    }
}
