use std::{path::Path, fs::{read_to_string}};

pub mod historical;
pub mod backtest;
pub mod datetime;
pub mod ta;

#[derive(Debug)]
pub struct Stock {
    ticker: String,
    security_type: String,
    name: String,
    market_cap: u64,
    pe_ratio: f32,
    eps: f32,
    high52: f32,
    low52: f32,
    historical_data: Vec<historical::HistoricalData>,
}

#[allow(dead_code)]
impl Stock {
    pub fn new(ticker: &str) -> Self {
        return Self {
            name: "".to_string(),
            ticker: ticker.to_string(),
            security_type: "".to_string(),
            market_cap: 0,
            pe_ratio: 0.0,
            eps: 0.0,
            high52: 0.0,
            low52: 0.0,
            historical_data: Vec::new()
        }
    }

    pub fn get_ticker(&self) -> String {return self.ticker.to_string();}
    pub fn set_ticker(&mut self, ticker: &str) {self.ticker = ticker.to_string();}

    pub fn get_security_type(&self) -> String {return self.security_type.to_string();}
    pub fn set_security_type(&mut self, security_type: &str) {self.security_type = security_type.to_string();}

    pub fn get_name(&self) -> String {return self.name.to_string();}
    pub fn set_name(&mut self, name: &str) {self.name = name.to_string();}

    pub fn get_market_cap(&self) -> u64 {return self.market_cap;}
    pub fn set_market_cap(&mut self, market_cap: u64) {self.market_cap = market_cap;}

    pub fn get_pe_ratio(&self) -> f32 {return self.pe_ratio;}
    pub fn set_pe_ratio(&mut self, pe_ratio: f32) {self.pe_ratio = pe_ratio;}

    pub fn get_eps(&self) -> f32 {return self.eps;}
    pub fn set_eps(&mut self, eps: f32) {self.eps = eps;}

    pub fn get_high52(&self) -> f32 {return self.high52;}
    pub fn set_high52(&mut self, high52: f32) {self.high52 = high52;}

    pub fn get_low52(&self) -> f32 {return self.low52;}
    pub fn set_low52(&mut self, low52: f32) {self.low52 = low52;}

    pub fn get_historical_data(&self) -> &Vec<historical::HistoricalData> {return &self.historical_data;}
    pub fn load_historical_data(&mut self, location: &Path) -> Result<bool, String> {
        if !location.exists() {
            return Err(format!("Could not find location for historical data for ticker {}: {:?}", self.ticker, location));
        }

        // Extract the content
        let content = match read_to_string(location) {
            Ok(contents) => contents,
            Err(error) => return Err(error.to_string())
        };

        // Add HistoricalData to Stock
        let content = content.replace("\r", "");
        let contents: Vec<&str> = content.split('\n').collect();
        for line in &contents[1..contents.len()] {
            let values: Vec<&str> = line.split(',').collect();
            self.historical_data.push(historical::HistoricalData::new(
                values[0],
                match values[1].to_string().parse() {
                    Ok(open) => open,
                    Err(_) => 0.0,
                },
                match values[2].to_string().parse() {
                    Ok(high) => high,
                    Err(_) => 0.0,
                },
                match values[3].to_string().parse() {
                    Ok(low) => low,
                    Err(_) => 0.0,
                },
                match values[4].to_string().parse() {
                    Ok(close) => close,
                    Err(_) => 0.0,
                },
                match values[5].to_string().parse() {
                    Ok(volume) => volume,
                    Err(_) => 0,
                },
            ));
        }

        // Sort data
        historical::sort_by_date(&mut self.historical_data);

        return Ok(true);
    }

    pub fn query_historical_data(&self, start_date: &datetime::DateTime, end_date: &datetime::DateTime) -> Vec<&historical::HistoricalData> {
        let mut o: Vec<&historical::HistoricalData> = Vec::new();
        for r in self.historical_data.iter() {
            if r.get_date().is_on_or_after(start_date) && r.get_date().is_on_or_before(end_date) {
                o.push(r);
            }
        }
        return o;
    }

    pub fn load_data(&mut self, location: &Path) -> Result<bool, String> {
        if !location.exists() {
            return Err(format!("Could not find location for data for ticker {}: {:?}", self.ticker, location));
        }

         // Extract the content
         let content = match read_to_string(location) {
            Ok(contents) => contents,
            Err(error) => return Err(error.to_string())
        };

        // Build entries
        let content: Vec<&str> = content.split('\n').collect();
        let values: Vec<&str> = content[1].split(',').collect();

        self.set_security_type(values[0]);
        self.set_market_cap(match values[1].to_string().parse() {
            Ok(market_cap) => market_cap,
            Err(_) => 0,
        });
        self.set_high52(match values[2].to_string().parse() {
            Ok(high52) => high52,
            Err(_) => 0.0,
        });
        self.set_low52(match values[3].to_string().parse() {
            Ok(low52) => low52,
            Err(_) => 0.0,
        });
        self.set_pe_ratio(match values[4].to_string().parse() {
            Ok(pe_ratio) => pe_ratio,
            Err(_) => 0.0,
        });
        self.set_eps(match values[5].to_string().parse() {
            Ok(eps) => eps,
            Err(_) => 0.0,
        });
        

        return Ok(true);
    }

    pub fn backtest(&self) {
        backtest::run(self);
    }

    pub fn to_string(&self) -> String {
        let mut output = "".to_owned();
        output.push_str("{",);
        output.push_str(&format!("\"ticker\": \"{}\",", self.ticker));
        output.push_str(&format!("\"security_type\": \"{}\",", self.security_type));
        output.push_str(&format!("\"name\": \"{}\",", self.name));
        output.push_str(&format!("\"market_cap\": {},", self.market_cap));
        output.push_str(&format!("\"pe_ratio\": {},", self.pe_ratio));
        output.push_str(&format!("\"eps\": {},", self.eps));
        output.push_str(&format!("\"high52\": {},", self.high52));
        output.push_str(&format!("\"low52\": {},", self.low52));
        output.push_str("\"historical_data\": [");
        for d in self.historical_data.iter() {
            output.push_str(&format!("{},", d.to_string()));
        }
        // Remove the last comma
        output.pop();
        output.push_str("]}");
        return output;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = Stock::new("spy");
        assert_eq!(s.get_ticker(), "spy");
    }

    #[test]
    fn test_load_contemporary_data() {
        let mut s= Stock::new("spy");
        match s.load_data(Path::new("./test/data/spy_contemporary.csv")) {Ok(b) => b, Err(error) => panic!("{}", error)};
        assert_eq!(s.get_ticker(), "spy");
        assert_eq!(s.get_high52(), 479.98);
        assert_eq!(s.get_low52(), 380.54);
        assert_eq!(s.get_market_cap(), 369696299325);
        assert_eq!(s.get_pe_ratio(), 0.0);
        assert_eq!(s.get_eps(), 0.0);
        assert_eq!(s.get_security_type(), "etf");
    }

    #[test]
    fn test_load_historical_data() {
        let mut s= Stock::new("spy");
        match s.load_historical_data(Path::new("./test/data/spy_historical.csv")) {Ok(b) => b, Err(error) => panic!("{}", error)};
        let hd = s.get_historical_data();
        assert_eq!(hd.len(), 5);
        assert_eq!(hd[0].get_date().to_string(), "1/2/2014 16:00:00");
        assert_eq!(hd[0].get_low(), 182.48);
        assert_eq!(hd[0].get_high(), 184.07);
        assert_eq!(hd[0].get_open(), 183.98);
        assert_eq!(hd[0].get_close(), 182.92);
        assert_eq!(hd[0].get_volume(), 119636836);
    }
}