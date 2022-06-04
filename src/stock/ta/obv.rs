/// Calculate the on-balance volume (OBV) based of a `Vec<f32>` of price data
/// and a `Vec<u32>` of volume data.
/// 
/// ### Definition
/// A momentum indicator used to predict price changes in a security using data
/// about volume.
/// 
/// ### Formula
/// `days_volume` conditions:
/// - `volume` if close > close_prev
/// - `0` if close = close_prev
/// - `-volume` if close < close_prev
/// 
/// `obv = obv_prev + days_volume`
/// 
/// ### Usage
/// Volume is often used to track large, institutional investors. OBV can 
/// help identify when institutions have decided a good point to buy up
/// retail investor sell offs. OBV is prone to producing false signals. By
/// leveraging moving averages (lagging signals) OBV may be balanced out to
/// confirm breakouts (when MA and OBV move in same direction).
/// 
/// NOTE: A large volume spike on a single day can throw off the OBV for
/// a while.
/// 
/// # Arguments
/// * `prices` - `Vec<f32>` containing prices for a period of time
/// * `volume` - `Vec<u32>` containing volume data for a period of time
/// 
/// ### Example
/// ```
/// obv::run(prices, volume);
/// obv::run(prices, volume);
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