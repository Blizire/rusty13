pub fn rot13(s : &String) -> String {
    let mut cypher_string = String::new();

    for c in s.chars() {
        // cast char to u8 (only works for ascii)
        let mut rot13_c = c as u8;

        // skip non typical ascii characters
        if rot13_c < 32 || rot13_c > 126 {
            continue;
        }

        // add 16 while casted to u8 then convert back
        rot13_c += 13;
        cypher_string.push(rot13_c as char);
    }

    cypher_string
}