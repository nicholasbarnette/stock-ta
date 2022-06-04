use crate::stock::ta::sma;
/// Calculate the exponential moving average (EMA) of a `Vec<f32>` of price data.
/// 
/// ### Definition
/// Average price of an asset sampled over a given period of time. Unlike
/// the simple moving average (SMA), more recent prices will incur a higher
/// weight. Thus, the EMA reacts more to recent price
/// fluctuations.
/// 
/// ### Formula
/// `s = 2 / (1 + p)`
/// 
/// `ema = (v * s) + (ema_prev * (1 - s))`
/// 
/// - `v`: value (latest period)
/// - `s`: smoothing coefficient
/// - `ema_prev`: previous period's ema
/// - `p`: number of periods being avgeraged
/// 
/// NOTE: The first reading of the EMA uses the SMA of the first `p` periods.
/// 
/// ### Usage
/// When a security price crosses the 200-day EMA, this generally signals a reversal.
/// Shorter term EMAs can be used to determine trading biases.
/// 
/// #### Short Term
/// - 8-day and 20-day EMA
/// 
/// #### Long Term
/// - 50-day and 200-day EMA
/// 
/// # Arguments
/// * `prices` - `Vec<f32>` containing prices for a period of time
/// * `periods` - Number of periods to average
/// 
/// ### Example
/// ```
/// ema::run(prices, 50);
/// ema::run(prices, 200);
/// ```
/// 
/// #### Resources
/// - https://www.investopedia.com/terms/e/ema.asp
pub fn run(prices: Vec<f32>, periods: usize) -> Vec<f32> {
    if prices.len() < periods+1 { panic!("Not enough entries to calculate the EMA. Received {}, but required {} (periods+1).", prices.len(), periods+1); }
    let smoothing: f32 = 2.0 / (periods as f32 + 1.0);
    let mut emas: Vec<f32> = Vec::new();
    let mut ema_prev = match sma::run(prices.to_vec(), periods).pop() {
        Some(v) => v,
        None => 0.0,
    };
    for i in periods..prices.len() {
        let ema = (prices[i] * smoothing) + (ema_prev * (1.0 - smoothing));
        ema_prev = ema;
        emas.push(ema);
    }
    return emas;
}