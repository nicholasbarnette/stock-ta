use std::{io::Write, fs::{File, read_dir, metadata, read_to_string}, path::Path, ffi::OsStr};

#[derive(Debug)]
struct Entry {
    date: String,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    volume: u32,
}

impl Entry {
    fn get_date(&self) -> String {
        return self.date.to_string();
    }
    fn get_open(&self) -> f32 {
        return self.open;
    }
    fn get_high(&self) -> f32 {
        return self.high;
    }
    fn get_low(&self) -> f32 {
        return self.low;
    }
    fn get_close(&self) -> f32 {
        return self.close;
    }
    fn get_volume(&self) -> u32 {
        return self.volume;
    }
    fn to_string(&self) -> String {
        let mut output = "".to_owned();
        output.push_str("{",);
        output.push_str(&format!("\"date\": \"{}\",", self.get_date()));
        output.push_str(&format!("\"close\": {},", self.get_close()));
        output.push_str(&format!("\"high\": {},", self.get_high()));
        output.push_str(&format!("\"low\": {},", self.get_low()));
        output.push_str(&format!("\"open\": {},", self.get_open()));
        output.push_str(&format!("\"volume\": {}", self.get_volume()));
        output.push_str("}");
        return output;
    }
}


fn unpack_csv_file(location: &Path) -> Result<(String, Vec<Entry>), String> {
    // Ensure the file exists
    if !location.exists() {return Err(format!("File does not exist: {:?}", location))}

    // Ensure file is a .csv format
    match location.extension() {
        Some(ext) => if !(ext == "csv") {return Ok(("".to_string(), Vec::new()))} else {},
        None => return Err("Could not get the extension.".to_string()),
    };

    // Get the name of the file without the extension
    let filename = location.file_stem().unwrap_or(OsStr::new("")).to_str();
    let filename = match filename {
        Some(filename) => filename,
        None => return Err(format!("Could not get filename: {:?}", filename)),
    };

    // Read the file and build the entries
    let contents = match read_to_string(location) {
        Ok(contents) => contents,
        Err(error) => return Err(error.to_string())
    };
    let entries = build_entries(&contents);

    return Ok((filename.to_string(), entries));
}


fn unpack_directory(location: &Path) -> Result<Vec<(String, Vec<Entry>)>, String> {
    // Ensure the directory exists
    if !location.exists() {return Err(format!("Directory does not exist: {:?}", location))}

    // Read the files
    let files = match read_dir(location) {
        Ok(files) => files,
        Err(error) => return Err(format!("Could not read files in directory: {:?}\n{:?}", location, error)),
    };

    // Iterate through the files
    let mut all_entries: Vec<(String, Vec<Entry>)> = Vec::new();
    for file in files {
        let file = match file {
            Ok(file) => file,
            Err(error) => return Err(format!("{:?}", error)),
        };
        match unpack_csv_file(&file.path()) {
            Ok(t) => all_entries.push(t),
            Err(error) => return Err(format!("Could not read file: {:?}", error)),
        };
    }

    return Ok(all_entries);
}

fn build_entries(contents: &str) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();

    // Build entries
    let contents: Vec<&str> = contents.split('\n').collect();
    for line in contents {
        let values: Vec<&str> = line.split(',').collect();

        let open = match values[1].to_string().parse() {
            Ok(open) => open,
            Err(_) => 0.0,
        };

        let high = match values[2].to_string().parse() {
            Ok(high) => high,
            Err(_) => 0.0,
        };

        let low = match values[3].to_string().parse() {
            Ok(low) => low,
            Err(_) => 0.0,
        };

        let close = match values[4].to_string().parse() {
            Ok(close) => close,
            Err(_) => 0.0,
        };

        let volume = match values[5].to_string().parse() {
            Ok(volume) => volume,
            Err(_) => 0,
        };
        
        entries.push(Entry {
            date: values[0].to_string(),
            close: close,
            open: open,
            high: high,
            low: low,
            volume: volume,
        });
    }

    return entries;
}

fn tuple2json(entries: Vec<(String, Vec<Entry>)>) -> String {
    let mut output = "{".to_owned();

    for entry in entries {
        output.push_str(&format!("\"{}\": ", entry.0));
        output.push_str("[");
        for e in entry.1 {
            output.push_str(&format!("{},", &e.to_string()));
        }
         // Pop off the last comma
        output.pop();
        output.push_str("],")
    }
    
    // Pop off the last comma
    output.pop();
    output.push_str("}");
    return output;
}


pub fn csv2json(location: &Path) {
    // Get metadata to determine if location is a file
    let md = metadata(location);
    let md = match md {
        Ok(md) => md,
        Err(error) => panic!("Could not get metadata for location: {:?}\n{:?}", location, error),
    };
    
    // Read all info into a tuple
    let mut all_tuples: Vec<(String, Vec<Entry>)> = Vec::new();
    if !md.is_dir() {
        match unpack_csv_file(location) {
            Ok(t) => if !(t.0 == "") {all_tuples.push(t)},
            Err(error) => println!("Could not build tuple: {:?}", error)
        };
    } else {
        match unpack_directory(location) {
            Ok(t) => all_tuples.extend(t),
            Err(error) => println!("Could not build tuple: {:?}", error)
        };
    }

    // Aggregate metadata to a single file
    let output = tuple2json(all_tuples);
    
    let mut output_file = File::create("tmp.json").unwrap();
    write!(output_file, "{}", output).unwrap();
}