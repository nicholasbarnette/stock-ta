use crate::stock::ta::ema;
/// Calculate the moving average convergence divergence (MACD) of a `Vec<f32>` of price data.
/// 
/// ### Definition
/// This indicator gives a picture of the momentum of a given security by displaying the
/// difference between two moving averages.
/// 
/// ### Formula
/// `macd = 12-day EMA - 26-day EMA`
/// 
/// ### Usage
/// When the MACD runs above the signal line, momentum is bullish. If the MACD runs below
/// the signal line, momentum is bearish. When the MACD and the signal line cross, a 
/// reversal may be signified. Sometimes this cross is a false positive and no reversal
/// occurs. It also does not predict all reversals.
/// 
/// In a long-term upward trend, if the MACD crosses the signal line to the upside after
/// a brief correction, this signifies a bullish confirmation. Similarly, in a long-term
/// downward trend, if the MACD crosses the signal line to the downside after a short
/// buy-up, this signals a bearish confirmation.
/// 
/// When the MACD forms two highs or lows that do not coorespond to to the price's
/// corresponding highs or lows, this signifies divergence. If, in a long-term bullish
/// sequence, the MACD forms two increasing lows while the price forms two decreasing
/// lows, this signifies a possible bullish divergence. Alternatively, if, in a long-
/// term bearish sequence, the MACD forms two decreasing highs while the price forms
/// two increasing highs, this signifies a possible bearish divergence. 
///
/// 
/// # Arguments
/// * `prices` - `Vec<f32>` containing prices for a period of time
/// 
/// # Returns
/// `(Vec<f32>, Vec<f32>)` containing values in the form of `(MACD, Signal)`.
/// 
/// ### Example
/// ```
/// macd::run(prices);
/// macd::run(prices);
/// ```
/// 
/// #### Resources
/// - https://www.investopedia.com/terms/m/macd.asp
pub fn run(prices: Vec<f32>) -> (Vec<f32>, Vec<f32>) {
    const LOW_PERIOD: usize = 12;
    const HIGH_PERIOD: usize = 26;
    
    let mut macd: Vec<f32> = Vec::new();
    // EMA calculation requires days+1 entries
    for i in HIGH_PERIOD+1..prices.len() {
        let emas12 = ema::run(prices[i-1-HIGH_PERIOD..i].to_vec(), LOW_PERIOD);
        let emas26 = ema::run(prices[i-1-HIGH_PERIOD..i].to_vec(), HIGH_PERIOD);
        macd.push(emas12[emas12.len()-1] - emas26[emas26.len()-1]);
    }
    let signal = ema::run(macd.to_vec(), 9);
    return (macd, signal);
}