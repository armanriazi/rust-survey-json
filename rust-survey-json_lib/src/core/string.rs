pub fn concat_two_string(str1: String, str2: String) -> String {
    let str = &mut str1.to_string().to_owned();
    str.push_str(&str2);
    let output: String = str.chars().filter(|c| !c.is_whitespace()).collect();
    output
}
