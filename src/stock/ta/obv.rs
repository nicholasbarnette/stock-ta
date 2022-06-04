/// Calculate the on-balance volume (OBV) based of a `Vec<f32>` of price data
/// and a `Vec<u32>` of volume data.
/// 
/// ### Definition
/// A momentum indicator used to predict price changes in a security using data
/// about volume.
/// 
/// ### Formula
/// volume conditions:
/// - `volume` if close > close_prev
/// - `0` if close = close_prev
/// - `-volume` if close < close_prev
/// 
/// `obv = obv_prev + volume`
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
/// obv::run(prices, 50);
/// obv::run(prices, 200);
/// ```
/// 
/// #### Resources
/// - https://www.investopedia.com/terms/o/onbalancevolume.asp
pub fn run(prices: Vec<f32>, volume: Vec<u32>) -> Vec<i32> {
    if prices.len() != volume.len() {panic!("Length mismatch. `prices` contains {} entries, but `volume` contains {}.", prices.len(), volume.len())}
    if prices.len() < 2 { panic!("Not enough entries to calculate OBV. Received {}, but required 2.", prices.len()); }
    let mut obvs: Vec<i32> = Vec::new();
    let mut obv_prev = 0;
    let mut price_prev = match prices.get(0) {
        Some(&v) => v,
        None => 0.0
    };
    for i in 1..prices.len() {
        let price = match prices.get(i) {
            Some(&v) => v,
            None => 0.0
        };
        let v = match volume.get(i) {
            Some(&v) => v,
            None => 0,
        };
        let mut v_final: i32 = 0;
        if price > price_prev { v_final = v as i32;}
        if price < price_prev { v_final = -1 * v as i32;}
        let obv = obv_prev + v_final;
        obvs.push(obv);
        obv_prev = obv;
        price_prev = price;
    }
    return obvs;
}