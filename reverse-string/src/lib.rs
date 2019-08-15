pub fn reverse(input: &str) -> String {
    let mut output = String::new();
    let mut inputchars: Vec<char> = Vec::new();
    for c in input.chars() {
        inputchars.push(c);
    }
    inputchars.reverse();
    for c in inputchars.iter() {
        output.push(*c);
    }
    output
}
