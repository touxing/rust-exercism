#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    // unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
    let superlist = second_list.is_empty()
        || first_list
            .windows(second_list.len())
            .any(|sl| sl == second_list);

    let sublist = first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|sl| sl == first_list);

    match (superlist, sublist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
        (false, true) => Comparison::Sublist,
    }
}
