use regex::Regex;

#[derive(Debug)]
pub enum JSONObject {
    String(String),
    Number(i32),
    Float(f32),
    Boolean(bool),
    Null(),
    Object(JSON),
    Array(Vec<JSONObject>)
}

#[derive(Debug)]
pub struct JSON {
    __root__: Vec<(String, JSONObject)>,
}


impl JSON {
    fn new() -> Self {
        return Self { __root__: Vec::new() };
    }

    fn _convert_kv<'a>(&mut self, key: &'a str, value: &'a str) -> (&'a str, JSONObject) {
        let re_object = Regex::new(r"^\{(?P<s>.*)\}$").unwrap();
        let re_array = Regex::new(r"^\[(?P<s>.*)\]$").unwrap();
        let re_number = Regex::new(r"^-?\d+$").unwrap();
        let re_float = Regex::new(r"^-?[\d\.]+$").unwrap();
        let re_string = Regex::new(r#"^".*"$"#).unwrap();
        let re_boolean = Regex::new(r"^(true|false)$").unwrap();
        let re_null = Regex::new(r"^(null)$").unwrap();

        if re_object.is_match(value) {
            let mut tmp = JSON::new();
            tmp.parse(value);
            return (key, JSONObject::Object(tmp));
        }

        if re_string.is_match(value) {
            return (key, JSONObject::String(value.to_string()));
        } else if re_boolean.is_match(value) {
            return (key, JSONObject::Boolean(match value { "true" => true, _ => false }));
        } else if re_number.is_match(value) {
            return (key, JSONObject::Number(value.parse::<i32>().unwrap()));
        } else if re_float.is_match(value) {
            return (key, JSONObject::Float(value.parse::<f32>().unwrap()));
        } else if re_null.is_match(value) {
            return (key, JSONObject::Null());
        } else {
            return (key, JSONObject::Null());
        }
    }

    fn _parse(&mut self, string: &str)  {

        let re_object = Regex::new(r"^\{(?P<s>.*)\}$").unwrap();
        let re_array = Regex::new(r"^\[(?P<s>.*)\]$").unwrap();
        let re_number = Regex::new(r"^-?\d+$").unwrap();
        let re_float = Regex::new(r"^-?[\d\.]+$").unwrap();
        let re_string = Regex::new(r#"^".*"$"#).unwrap();
        let re_boolean = Regex::new(r"^(true|false)$").unwrap();
        let re_null = Regex::new(r"^(null)$").unwrap();
        let re_pv = Regex::new(r#""([^"]+)":("[^"]*"|\d+|true|false|(?:\{.*\})|(?:\[.*\])),"#).unwrap();

        if re_object.is_match(string) {
            let s = match re_object.captures(string).unwrap().get(1) {
                Some(s) => s.as_str(),
                None => "",
            };
            let s = &format!("{},", s);
            for el in re_pv.captures_iter(s) {
                let kv = self._convert_kv(&el[1], &el[2]);
                self.push(kv.0, kv.1);
            }
        } else if re_array.is_match(string) {
            let s = match re_array.captures(string).unwrap().get(1) {
                Some(s) => s.as_str(),
                None => "",
            };
            let s = &format!("{},", s);
            for el in re_pv.captures_iter(s) {
                let kv = self._convert_kv(&el[1], &el[2]);
                self.push(kv.0, kv.1);
            }
        } else {
            let kv = self._convert_kv("__root__", string);
            self.push(kv.0, kv.1);
        }
    }

    pub fn parse(&mut self, string: &str) {
        let s = sanitize(string);
        self._parse(&s);
    }

    pub fn push(&mut self, key: &str, value: JSONObject) {
        self.__root__.push(
            (key.to_string(), value)
        );
    }

    pub fn find(&self, key: &str) -> Result<&JSONObject, String> {
        for i in self.__root__.iter() {
            if i.0 == key { 
                return Ok(&i.1);
            }
        }
        return Err(format!("Could not find key: `{}`", key));
    }

    pub fn to_string(&self) -> String {
        return format!("{:?}", self.__root__);
    }
}

pub fn serialize(string: &str) -> JSON {
    let mut json = JSON::new();
    json.parse(string);
    return json;
}





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