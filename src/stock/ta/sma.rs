/// Calculate the simple moving average (SMA) based on a `Vec<f32>` of price data.
/// 
/// ### Definition
/// Average price of an asset sampled over a given period of time. Used
/// to quickly determine if price action is trending up or down over a
/// longer period of time.
/// 
/// ### Formula
/// `sma = (p_0 + p_1 + ... + p_n) / n`
/// 
/// - `a_n`: price of asset at given period
/// - `n`: number of periods to average
/// 
/// ### Usage
/// - death cross: 50-day SMA crosses below 200-day SMA
/// - golden cross: 200-day SMA crosses below 50-day SMA
/// 
/// # Arguments
/// * `prices` - `Vec<f32>` containing prices for a period of time
/// * `periods` - Number of periods to average
/// 
/// ### Example
/// ```
/// sma::run(prices, 50);
/// sma::run(prices, 200);
/// ```
/// 
/// #### Resources
/// - https://www.investopedia.com/terms/s/sma.asp
pub fn run(prices: Vec<f32>, periods: usize) -> Vec<f32> {
    if prices.len() < periods { panic!("Not enough entries to calculate the SMA. Received {}, but required {}.", prices.len(), periods); }
    let mut smas: Vec<f32> = Vec::new();
    for i in periods-1..prices.len() {
        let sum = prices[i+1-periods..i].iter().sum::<f32>();
        smas.push(sum / periods as f32);
    }
    return smas;
}