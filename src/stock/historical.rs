use crate::stock::datetime::{DateTime};

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

fn partition(a: &mut Vec<HistoricalData>, low: i64, high: i64) -> i64 {
    let pivot = match a.get(high as usize) {
        Some(el) => el.clone(),
        _ => panic!("Array index {} out of bounds.", high-1)
    };
    let mut i = low - 1;
    for j in low..high {
        if a[j as usize].get_date().is_before(&pivot.get_date()) {
            i += 1;
            a.swap(i as usize, j as usize);
        }
    }
    a.swap((i+1) as usize, high as usize);
    return i+1;
}

fn _quicksort(a: &mut Vec<HistoricalData>, low: i64, high: i64) {
    if low < high {
        let pi = partition(a, low, high);
        _quicksort(a, low, pi-1);
        _quicksort(a, pi+1, high);
    }
}

fn quicksort(a: &mut Vec<HistoricalData>) {
    _quicksort(a, 0, (a.len() as i64)-1);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let d = HistoricalData::new("4/23/2021 16:00:00", 10.0, 17.0, 8.0, 11.0, 10050);
        assert_eq!(d.get_date().get_year(), 2021);
        assert_eq!(d.get_date().get_month(), 4);
        assert_eq!(d.get_date().get_day(), 23);
        assert_eq!(d.get_date().get_hour(), 16);
        assert_eq!(d.get_date().get_minute(), 0);
        assert_eq!(d.get_date().get_second(), 0);
        assert_eq!(d.get_open(), 10.0);
        assert_eq!(d.get_high(), 17.0);
        assert_eq!(d.get_low(), 8.0);
        assert_eq!(d.get_close(), 11.0);
        assert_eq!(d.get_volume(), 10050);
    }

    #[test]
    fn test_sort_by_date_simple() {
        let d1 = HistoricalData::new("4/23/2021 16:00:00", 10.0, 17.0, 8.0, 11.0, 10050);
        let d2 = HistoricalData::new("4/23/2020 16:00:00", 10.0, 17.0, 8.0, 11.0, 10050);
        let d3 = HistoricalData::new("4/23/2019 16:00:00", 10.0, 17.0, 8.0, 11.0, 10050);
        let d4 = HistoricalData::new("4/23/2018 16:00:00", 10.0, 17.0, 8.0, 11.0, 10050);
        let d5 = HistoricalData::new("4/23/2017 16:00:00", 10.0, 17.0, 8.0, 11.0, 10050);
        let mut dates = vec![d1,d2,d3,d4,d5];
        sort_by_date(&mut dates);
        assert_eq!(dates[0].get_date().is_before(&dates[1].get_date()), true);
        assert_eq!(dates[1].get_date().is_before(&dates[2].get_date()), true);
        assert_eq!(dates[2].get_date().is_before(&dates[3].get_date()), true);
        assert_eq!(dates[3].get_date().is_before(&dates[4].get_date()), true);
    }

    #[test]
    fn test_sort_by_date_complex() {
        let d1 = HistoricalData::new("4/23/2021 16:00:00", 10.0, 17.0, 8.0, 11.0, 10050);
        let d2 = HistoricalData::new("4/23/2019 16:00:00", 10.0, 17.0, 8.0, 11.0, 10050);
        let d3 = HistoricalData::new("4/23/2018 16:00:00", 10.0, 17.0, 8.0, 11.0, 10050);
        let d4 = HistoricalData::new("4/23/2017 16:00:00", 10.0, 17.0, 8.0, 11.0, 10050);
        let d5 = HistoricalData::new("4/23/2020 16:00:00", 10.0, 17.0, 8.0, 11.0, 10050);
        let mut dates = vec![d1,d2,d3,d4,d5];
        sort_by_date(&mut dates);
        assert_eq!(dates[0].get_date().is_before(&dates[1].get_date()), true);
        assert_eq!(dates[1].get_date().is_before(&dates[2].get_date()), true);
        assert_eq!(dates[2].get_date().is_before(&dates[3].get_date()), true);
        assert_eq!(dates[3].get_date().is_before(&dates[4].get_date()), true);
    }

    #[test]
    fn test_sort_by_time_simple() {
        let d1 = HistoricalData::new("4/23/2020 16:30:50", 10.0, 17.0, 8.0, 11.0, 10050);
        let d2 = HistoricalData::new("4/23/2020 16:30:40", 10.0, 17.0, 8.0, 11.0, 10050);
        let d3 = HistoricalData::new("4/23/2020 16:30:30", 10.0, 17.0, 8.0, 11.0, 10050);
        let d4 = HistoricalData::new("4/23/2020 16:30:20", 10.0, 17.0, 8.0, 11.0, 10050);
        let d5 = HistoricalData::new("4/23/2020 16:30:10", 10.0, 17.0, 8.0, 11.0, 10050);
        let mut dates = vec![d1,d2,d3,d4,d5];
        sort_by_date(&mut dates);
        assert_eq!(dates[0].get_date().is_before(&dates[1].get_date()), true);
        assert_eq!(dates[1].get_date().is_before(&dates[2].get_date()), true);
        assert_eq!(dates[2].get_date().is_before(&dates[3].get_date()), true);
        assert_eq!(dates[3].get_date().is_before(&dates[4].get_date()), true);
    }

    #[test]
    fn test_sort_by_time_complex() {
        let d1 = HistoricalData::new("4/23/2020 16:30:30", 10.0, 17.0, 8.0, 11.0, 10050);
        let d2 = HistoricalData::new("4/23/2020 16:30:40", 10.0, 17.0, 8.0, 11.0, 10050);
        let d3 = HistoricalData::new("4/23/2020 16:30:50", 10.0, 17.0, 8.0, 11.0, 10050);
        let d4 = HistoricalData::new("4/23/2020 16:30:10", 10.0, 17.0, 8.0, 11.0, 10050);
        let d5 = HistoricalData::new("4/23/2020 16:30:20", 10.0, 17.0, 8.0, 11.0, 10050);
        let mut dates = vec![d1,d2,d3,d4,d5];
        sort_by_date(&mut dates);
        assert_eq!(dates[0].get_date().is_before(&dates[1].get_date()), true);
        assert_eq!(dates[1].get_date().is_before(&dates[2].get_date()), true);
        assert_eq!(dates[2].get_date().is_before(&dates[3].get_date()), true);
        assert_eq!(dates[3].get_date().is_before(&dates[4].get_date()), true);
    }
}