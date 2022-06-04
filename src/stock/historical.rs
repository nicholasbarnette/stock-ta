use crate::stock::datetime::{DateTime, compare};

#[derive(Debug)]
#[derive(Clone)]
pub struct HistoricalData {
    date: DateTime,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    volume: u32,
}

#[allow(dead_code)]
impl HistoricalData {
    pub fn new(date: &str, open: f32, high: f32, low: f32, close: f32, volume: u32) -> Self {
        return Self {
            date: DateTime::new(date),
            open: open,
            high: high,
            low: low,
            close: close,
            volume: volume,
        };
    }

    pub fn get_date(&self) -> &DateTime { return &self.date; }
    pub fn set_date(&mut self, date: &str) { self.date = DateTime::new(date); }

    pub fn get_open(&self) -> f32 { return self.open; }
    pub fn set_open(&mut self, open: f32) { self.open = open; }

    pub fn get_high(&self) -> f32 { return self.high; }
    pub fn set_high(&mut self, high: f32) { self.high = high; }

    pub fn get_low(&self) -> f32 { return self.low; }
    pub fn set_low(&mut self, low: f32) { self.low = low; }

    pub fn get_close(&self) -> f32 { return self.close; }
    pub fn set_close(&mut self, close: f32) { self.close = close; }

    pub fn get_volume(&self) -> u32 { return self.volume; }
    pub fn set_volume(&mut self, volume: u32) { self.volume = volume; }

    pub fn to_string(&self) -> String {
        let mut output = "".to_owned();
        output.push_str("{",);
        output.push_str(&format!("\"date\": \"{}\",", self.get_date().to_string()));
        output.push_str(&format!("\"close\": {},", self.get_close()));
        output.push_str(&format!("\"high\": {},", self.get_high()));
        output.push_str(&format!("\"low\": {},", self.get_low()));
        output.push_str(&format!("\"open\": {},", self.get_open()));
        output.push_str(&format!("\"volume\": {}", self.get_volume()));
        output.push_str("}");
        return output;
    }
}
pub fn sort_by_date(d: &mut Vec<HistoricalData>) {
    quicksort(d);
}

fn partition(a: &mut Vec<HistoricalData>, low: usize, high: usize) -> usize {
    let pivot = match a.get(high) {
        Some(el) => el.clone(),
        _ => panic!("Array index {} out of bounds.", high-1)
    };
    let mut i = low - 1;
    for j in low..high {
        match a.to_vec().get(j).clone() {
            Some(_v) => {
                if !compare(a[j].get_date(), pivot.get_date()) {
                    let _ = &a.swap(i, j);
                    i += 1;
                }
            }
            _ => {panic!("Array index {:?} for j out of bounds", j)}
        }
    }
    a.swap(i+1, high);
    return i+1;
}

fn _quicksort(a: &mut Vec<HistoricalData>, low: usize, high: usize) {
    if low > high {
        let pi = partition(a, low, high);
        _quicksort(a, low, pi-1);
        _quicksort(a, pi+1, high);
    }
}

fn quicksort(a: &mut Vec<HistoricalData>) {
    _quicksort(a, 0, a.len()-1);
}