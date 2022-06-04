use crate::stock::ta::sma;

/// Calculate the bollinger band based on a `Vec<f32>` of price data.
/// 
/// ### Definition
/// An indicator -- bounded between [0-100] -- comprised of the most recent
/// close price and a set number of a period's high/low prices to gauge a
/// security's current momentum and whether it is overbought/oversold.
/// 
/// ### Formula
/// - `bolu = 20sma + (2 * std_dev(20close))`
/// - `bolm = 20sma`
/// - `bold = 20sma - (2 * std_dev(20close))`
/// 
/// - `bolu`: upper bollinger band
/// - `bolm`: middle bollinger band
/// - `bold`: lower bollinger band
/// - `20sma`: 20-day simple moving average of closing price
/// - `std_dev(20close)`: standard deviation of the last 20 closing prices
/// 
/// ### Usage
/// A security can typically be considered overbought when its prices moves to
/// the upper bollinger band or oversold when its price moves to the lower
/// bollinger band. The distance between the upper and lower bands will increase
/// with increased volatility and decrease with decreased volatility.
/// 
/// Prices will occasionally break out of the area between the upper and lower
/// bands. This is *NOT* necessarily a buy/sell signal, but rather the result
/// of a major event (ex. earnings).
/// 
/// # Arguments
/// * `prices` - `Vec<f32>` containing prices for a period of time
/// 
/// ### Example
/// ```
/// bollinger_band::run(prices);
/// bollinger_band::run(prices);
/// ```
/// 
/// #### Resources
/// - https://www.investopedia.com/terms/b/bollingerbands.asp
pub fn run(prices: Vec<f32>) -> Vec<(f32, f32, f32)> {
    const PERIOD: usize = 20;
    if prices.len() < PERIOD { panic!("Not enough entries to calculate bollinger bands. Received {}, but required {}.", prices.len(), PERIOD); }
    let mut bbs: Vec<(f32, f32, f32)> = Vec::new();
    for i in PERIOD..prices.len() {
        let ma = sma::run(prices[i-PERIOD..i].to_vec(), PERIOD);
        let ma = match ma.get(0) {
            Some(&v) => v,
            None => panic!("Error calculating SMA.")
        };
        let std = std_dev(&prices);
        let bolu = ma + (2.0 * std);
        let bold = ma - (2.0 * std);
        bbs.push((bold, ma, bolu));
    }
    return bbs;
}


fn mean(data: &Vec<f32>) -> f32 {
    let sum: f32 = data.iter().sum();
    return sum / data.len() as f32;
}

fn std_dev(data: &Vec<f32>) -> f32 {
    let m = mean(data);
    let variance = data.iter().map(|d| {
        let diff = m - *d;
        diff * diff
    }).sum::<f32>() / data.len() as f32;
    return variance.sqrt();
}