/// Requires date to be formatted as `m/d/yyyy hh:mm:ss`.
/// 
/// # Example
/// ```
/// let d = "1/2/2014 16:00:00";
/// let d = "12/31/2020 5:31:22";
/// ```
#[derive(Debug)]
#[derive(Clone)]
pub struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

#[allow(dead_code)]
impl DateTime {
    pub fn new(datetime: &str) -> Self {
        // Regex captures are incredibly slow in Rust at the moment...
        // let re = Regex::new(r"^(\d{1,2})/(\d{1,2})/(\d{4}) (\d{1,2}):(\d{2}):(\d{2})$").unwrap();
        let mut i = 0;
        let mut month = "".to_owned();
        let mut day = "".to_owned();
        let mut year = "".to_owned();
        let mut hour = "".to_owned();
        let mut minute = "".to_owned();
        let mut second = "".to_owned();
        for c in datetime.chars() {
            if c == '/' || c == ' ' || c == ':' {
                i += 1;
            } else {
                match i {
                    0 => month.push_str(&format!("{}", c)),
                    1 => day.push_str(&format!("{}", c)),
                    2 => year.push_str(&format!("{}", c)),
                    3 => hour.push_str(&format!("{}", c)),
                    4 => minute.push_str(&format!("{}", c)),
                    5 => second.push_str(&format!("{}", c)),
                    _ => panic!("Extra characters found in datetime string \"{}\".", datetime)
                }
            }
        }

        return Self {
            month: month.parse::<u8>().unwrap(),
            day: day.parse::<u8>().unwrap(),
            year: year.parse::<u16>().unwrap(),
            hour: hour.parse::<u8>().unwrap(),
            minute: minute.parse::<u8>().unwrap(),
            second: second.parse::<u8>().unwrap(),
        }
    }

    pub fn get_year(&self) -> u16 { return self.year; }
    pub fn get_month(&self) -> u8 { return self.month; }
    pub fn get_day(&self) -> u8 { return self.day; }
    pub fn get_hour(&self) -> u8 { return self.hour; }
    pub fn get_minute(&self) -> u8 { return self.minute; }
    pub fn get_second(&self) -> u8 { return self.second; }

    /// Compares two `DateTime`s and returns `true` if both `DateTime`s
    /// are equivalent.__rust_force_expr!
    /// 
    /// ### Examples
    /// ```
    /// let d1 = DateTime::new("4/23/2021 16:00:00");
    /// let d2 = DateTime::new("4/23/2021 16:00:00");
    /// assert_eq!(d1.is_on(&d2), true);
    /// assert_eq!(d2.is_on(&d1), true);
    /// ```
    pub fn is_on(&self, d: &DateTime) -> bool {
        if self.year == d.get_year() && self.month == d.get_month() && self.day == d.get_day() && self.hour == d.get_hour() && self.minute == d.get_minute() { return true; }
        return false;
    }

    /// Compares two `DateTime`s and returns `true` if the caller comes
    /// after the callee.
    /// 
    /// ```
    /// let d1 = DateTime::new("4/23/2021 16:00:00");
    /// let d2 = DateTime::new("4/23/2020 16:00:00");
    /// assert_eq!(d1.is_after(&d2), true);
    /// assert_eq!(d2.is_after(&d1), false);
    /// ```
    pub fn is_after(&self, d: &DateTime) -> bool {
        if self.year > d.get_year() { return true; }
        if self.year == d.get_year() && self.month > d.get_month() { return true; }
        if self.year == d.get_year() && self.month == d.get_month() && self.day > d.get_day() { return true; }
        if self.year == d.get_year() && self.month == d.get_month() && self.day == d.get_day() && self.hour > d.get_hour() { return true; }
        if self.year == d.get_year() && self.month == d.get_month() && self.day == d.get_day() && self.hour == d.get_hour() && self.minute > d.get_minute() { return true; }
        if self.year == d.get_year() && self.month == d.get_month() && self.day == d.get_day() && self.hour == d.get_hour() && self.minute == d.get_minute() && self.second > d.get_second() { return true; }
        return false;
    }

    /// Compares two `DateTime`s and returns `true` if the caller is on
    /// or comes after the callee.
    /// 
    /// ```
    /// let d1 = DateTime::new("4/23/2021 16:00:00");
    /// let d2 = DateTime::new("4/23/2020 16:00:00");
    /// assert_eq!(d1.is_after(&d2), true);
    /// assert_eq!(d2.is_after(&d1), false);
    /// ```
    pub fn is_on_or_after(&self, d: &DateTime) -> bool {
        if self.year >= d.get_year() { return true; }
        if self.year == d.get_year() && self.month >= d.get_month() { return true; }
        if self.year == d.get_year() && self.month == d.get_month() && self.day >= d.get_day() { return true; }
        if self.year == d.get_year() && self.month == d.get_month() && self.day == d.get_day() && self.hour >= d.get_hour() { return true; }
        if self.year == d.get_year() && self.month == d.get_month() && self.day == d.get_day() && self.hour == d.get_hour() && self.minute >= d.get_minute() { return true; }
        return false;
    }

    /// Compares two `DateTime`s and returns `true` if the caller comes
    /// before the callee.
    /// 
    /// ```
    /// let d1 = DateTime::new("4/23/2020 16:00:00");
    /// let d2 = DateTime::new("4/23/2021 16:00:00");
    /// assert_eq!(d1.is_after(&d2), true);
    /// assert_eq!(d2.is_after(&d1), false);
    /// ```
    pub fn is_before(&self, d: &DateTime) -> bool {
        if self.year < d.get_year() { return true; }
        if self.year == d.get_year() && self.month < d.get_month() { return true; }
        if self.year == d.get_year() && self.month == d.get_month() && self.day < d.get_day() { return true; }
        if self.year == d.get_year() && self.month == d.get_month() && self.day == d.get_day() && self.hour < d.get_hour() { return true; }
        if self.year == d.get_year() && self.month == d.get_month() && self.day == d.get_day() && self.hour == d.get_hour() && self.minute < d.get_minute() { return true; }
        return false;
    }

    /// Compares two `DateTime`s and returns `true` if the caller is on
    /// or comes before the callee.
    /// 
    /// ```
    /// let d1 = DateTime::new("4/23/2020 16:00:00");
    /// let d2 = DateTime::new("4/23/2021 16:00:00");
    /// assert_eq!(d1.is_after(&d2), true);
    /// assert_eq!(d2.is_after(&d1), false);
    /// ```
    pub fn is_on_or_before(&self, d: &DateTime) -> bool {
        if self.year <= d.get_year() { return true; }
        if self.year == d.get_year() && self.month <= d.get_month() { return true; }
        if self.year == d.get_year() && self.month == d.get_month() && self.day <= d.get_day() { return true; }
        if self.year == d.get_year() && self.month == d.get_month() && self.day == d.get_day() && self.hour <= d.get_hour() { return true; }
        if self.year == d.get_year() && self.month == d.get_month() && self.day == d.get_day() && self.hour == d.get_hour() && self.minute <= d.get_minute() { return true; }
        return false;
    }

    pub fn to_string(&self) -> String{
        return format!("{}/{}/{} {}:{}:{}", self.month, self.day, self.year, self.hour, self.minute, self.second);
    }
}

/// Compares two `DateTime`s and returns `true` if the first `DateTime` is
/// before the second. Returns `false` if `DateTime`s are exquivalent.
/// 
/// # Examples
/// ```
/// compare(DateTime::new("1/1/2020 12:00:00"), DateTime::new("1/1/2021 12:00:00")); // returns `true`
/// compare(DateTime::new("1/1/2020 12:00:00"), DateTime::new("12/31/2020 12:00:00")); // returns `true`
/// compare(DateTime::new("1/1/2020 12:00:00"), DateTime::new("1/1/2019 12:00:00")); // returns `false`
/// compare(DateTime::new("12/31/2020 12:00:00"), DateTime::new("1/1/2020 12:00:00")); // returns `false`
/// compare(DateTime::new("1/1/2020 12:00:00"), DateTime::new("1/1/2020 12:00:00")); // returns `false`
/// ```
pub fn compare(dt1: &DateTime, dt2: &DateTime) -> bool {
    return dt1.is_before(dt2);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let d = DateTime::new("4/23/2021 16:00:00");
        assert_eq!(d.get_year(), 2021);
        assert_eq!(d.get_month(), 4);
        assert_eq!(d.get_day(), 23);
        assert_eq!(d.get_hour(), 16);
        assert_eq!(d.get_minute(), 0);
        assert_eq!(d.get_second(), 0);
    }

    #[test]
    fn test_comparisons() {
        let d1 = DateTime::new("4/23/2021 16:00:00");
        let d2 = DateTime::new("4/23/2020 16:00:00");
        assert_eq!(d1.is_after(&d2), true);
        assert_eq!(d1.is_on_or_after(&d2), true);
        assert_eq!(d1.is_before(&d2), false);
        assert_eq!(d1.is_on_or_before(&d2), false);

        let d1 = DateTime::new("4/23/2021 16:00:00");
        let d2 = DateTime::new("4/23/2021 16:00:00");
        assert_eq!(d1.is_on_or_after(&d2), true);
        assert_eq!(d1.is_on_or_before(&d2), true);
        assert_eq!(d1.is_on(&d2), true);
    }
}