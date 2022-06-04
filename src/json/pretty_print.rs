/// Creates a pretty version of a JSON by adding in whitespace
/// to make the object more readable.
/// 
/// # Arguments
/// 
/// * `string` - A string to sanitize into a JSON string
/// 
/// # Examples
/// 
/// ```
/// let json_str = json::pretty_print("{\"property1\": \"value 1\",\"property2\": \"value 2\"}");
/// 
/// assert(
///     "{
///         \"property1\": \"value 1\",
///         \"property2\": \"value 2\"
///     }",
///     json_str
/// );
/// ```
pub fn pretty_print(string: &str) -> String {
    let mut in_str = false;
    let mut level = 0;
    let mut new_str = "".to_owned();
    let c_array: Vec<char> = string.chars().collect();
    for c in c_array {
        if !in_str {
            match c {
                '{' => {
                    level += 1;
                    new_str.push_str(&format!("{}\n{}", c, "\t".repeat(level)));
                },
                '[' => {
                    level += 1;
                    new_str.push_str(&format!("{}\n{}", c, "\t".repeat(level)));
                },
                '}' => {
                    level -= 1;
                    new_str.push_str(&format!("\n{}{}", "\t".repeat(level), c));
                },
                ']' => {
                    level -= 1;
                    new_str.push_str(&format!("\n{}{}", "\t".repeat(level), c));
                },
                ':' => new_str.push_str(&format!("{} ", c)),
                ',' => new_str.push_str(&format!("{}\n{}", c, "\t".repeat(level))),
                '"' => {
                    in_str = true;
                    new_str.push_str(&format!("{}", c));
                },
                _ => new_str.push_str(&format!("{}", c))
            };
        } else {
            match c {
                '"' => {
                    in_str = false;
                    new_str.push_str(&format!("{}", c));
                },
                _ => new_str.push_str(&format!("{}", c))
            };
        }
    }

    return new_str.to_string();
}