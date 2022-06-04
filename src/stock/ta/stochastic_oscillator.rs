

/// Calculate the stochasitc oscillator based on a `Vec<(f32, f32, f32)>` of
/// price data.
/// 
/// ### Definition
/// An indicator -- bounded between [0-100] -- comprised of the most recent
/// close price and a set number of a period's high/low prices to gauge a
/// security's current momentum and whether it is overbought/oversold.
/// 
/// ### Formula
/// `%K = ((c - l14) / (h14 - l14)) * 100`
/// 
/// - `%K`: current value of the stochastic indicator
/// - `c`: current price
/// - `h14`: highest price during last 14 trading sessions
/// - `l14`: lowest price during last 14 trading sessions
/// 
/// ### Usage
/// Typically, when the stochastic oscillator is greater than 80, the security
/// is possibly overbought. When the stochastic oscillaror is less than 20 the
/// security is possibly oversold. Indications that the security is overbought
/// or oversold are not necessarily indicitave of a reversal.
/// 
/// ##### Divergence
/// Divergence can be an important sign of an impending reversal. If the
/// security reaches a new lower low while the stochastic oscillator reaches
/// a higher low, then it might signify bearish exhaustion which may turn
/// into a bullish reversal.
/// 
/// # Arguments
/// * `prices` - `Vec<(f32, f32, f32)>` containing prices for a period of time
///              in the format of `Vec<(close, low, high)>`
/// 
/// ### Example
/// ```
/// stochastic_oscillator::run(prices);
/// stochastic_oscillator::run(prices);
/// ```
/// 
/// #### Resources
/// - https://www.investopedia.com/terms/s/stochasticoscillator.asp
pub fn run(prices: Vec<(f32, f32, f32)>) -> Vec<f32> {
    const PERIOD: usize = 14;
    if prices.len() < PERIOD { panic!("Not enough entries to calculate OBV. Received {}, but required {}.", prices.len(), PERIOD); }
    let mut oscs: Vec<f32> = Vec::new();

    for i in PERIOD-1..prices.len() {
        let cur = match prices.get(i) {
            Some(&v) => v,
            None => panic!("Could not get entry in `prices`."),
        };
        let p = cur.0;
        let mut low14 = cur.1;
        let mut high14 = cur.2;
        for j in i+1-PERIOD..i {
            let prev = match prices.get(j) {
                Some(&v) => v,
                None => panic!("Could not get entry in `prices`."),
            };
            if low14 > prev.1 { low14 = prev.1; }
            if high14 < prev.2 { high14 = prev.2; }
        }
        let osc = ((p - low14) / (high14 - low14)) * 100.0;
        oscs.push(osc);
    }
    return oscs;
}