pub fn brackets_are_balanced(string: &str) -> bool {
    let mut input:Vec<char> = vec![];

    for c in string.chars() {
        match c {
            '(' | '{' | '[' => input.push(c),

            ')' => if input.pop() != Some('(') {return false},
            '}' => if input.pop() != Some('{') {return false},
            ']' => if input.pop() != Some('[') {return false},
            _ => ()
        }
    }
    input.is_empty()
}
