use regex::Regex;

/// Removes unnecessary white space from the input `string` and
/// returns the new string as the output.
/// 
/// # Arguments
/// 
/// * `string` - A string to sanitize into a JSON string
/// 
/// # Examples
/// 
/// ```
/// let json_str = json::sanitize("{
///     \"property1\": \"value 1\",
///     \"property2\": \"value 2\"
/// }");
/// 
/// assert(
///     "{\"property1\":\"value 1\",\"property2\":\"value 2\"}",
///     json_str
/// );
/// ```
pub fn sanitize(string: &str) -> String {
    let re = Regex::new(r#"(?P<y>[\{\[\}\]:,]|"[^"]+"|\S+)"#).unwrap();
    let mut sanitized_string = "".to_owned();
    for m in re.captures_iter(string) {
        sanitized_string.push_str(&m[1]);
    }
    return sanitized_string;
}