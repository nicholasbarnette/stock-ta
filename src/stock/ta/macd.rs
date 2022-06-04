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
    if prices.len() < HIGH_PERIOD+1 { panic!("Not enough entries to calculate the EMA. Received {}, but required {} (26+1).", prices.len(), HIGH_PERIOD+1); }
    
    let mut macd: Vec<f32> = Vec::new();
    // EMA calculation requires `period+1` entries
    let mut i = HIGH_PERIOD+1;
    while {
        let emas12 = ema::run(prices[i-1-HIGH_PERIOD..i].to_vec(), LOW_PERIOD);
        let emas26 = ema::run(prices[i-1-HIGH_PERIOD..i].to_vec(), HIGH_PERIOD);
        macd.push(emas12[emas12.len()-1] - emas26[emas26.len()-1]);
        i += 1;
        i <= prices.len()
    } {}

    // Calculate the MACD signal line
    let mut signal: Vec<f32> = Vec::new();
    if macd.len() > 9 { signal = ema::run(macd.to_vec(), 9); };

    return (macd, signal);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_simple() {
        let prices = vec![
            10.0, 10.0, 15.0, 20.0, 20.0,
            10.0, 10.0, 15.0, 20.0, 20.0,
            10.0, 10.0, 15.0, 20.0, 20.0,
            10.0, 10.0, 15.0, 20.0, 20.0,
            10.0, 10.0, 15.0, 20.0, 20.0,
            10.0, 10.0
        ];
        assert_eq!(run(prices), (vec![-0.17376137], vec![]));
    }

    #[test]
    fn test_run_complex() {
        let prices = vec![
            10.0, 10.0, 15.0, 20.0, 20.0,
            12.0, 8.0, 20.0, 42.0, 36.0,
            11.0, 19.0, 3.0, 4.0, 7.0,
            10.0, 10.0, 15.0, 20.0, 20.0,
            12.0, 8.0, 20.0, 42.0, 36.0,
            11.0, 19.0, 3.0, 4.0, 7.0,
            10.0, 10.0, 15.0, 20.0, 20.0,
            12.0, 8.0, 20.0, 42.0, 36.0,
            11.0, 19.0, 3.0, 4.0, 7.0,
            10.0, 10.0, 15.0, 20.0, 20.0,
            12.0, 8.0, 20.0, 42.0, 36.0,
            11.0, 19.0, 3.0, 4.0, 7.0,
        ];
        assert_eq!(
            run(prices),
            (
                vec![
                    2.4991398, 0.9562206, -0.7649956, -1.7961569, -2.1694717,
                    -2.3433933, -2.3066406, -1.871275, -1.0476179, -0.308959,
                    -0.28025723, -0.09412098, 2.2728977, 3.9846954, 3.26997,
                    2.4991398, 0.9562206, -0.7649956, -1.7961569, -2.1694717,
                    -2.3433933, -2.3066406, -1.871275, -1.0476179, -0.308959,
                    -0.28025723, -0.09412098, 2.2728977, 3.9846954, 3.26997,
                    2.4991398, 0.9562206, -0.7649956, -1.7961569
                ],
                vec![
                    -0.84794205, -0.7344051, -0.60634834, -0.0304991, 0.7725398,
                    1.2720258, 1.5174487, 1.4052031, 0.97116345, 0.41769937,
                    -0.09973487, -0.54846656, -0.90010136, -1.094336, -1.0849924,
                    -0.9297857, -0.79988, -0.65872824, -0.07240304, 0.73901665,
                    1.2452073, 1.4959939, 1.3880392, 0.9574323, 0.40671447
                ]
            ));
    }

    #[test]
    #[should_panic(expected = "Not enough entries to calculate the EMA. Received 5, but required 27 (26+1).")]
    fn test_run_not_enough_elements() {
        run(vec![10.0, 10.0, 15.0, 20.0, 20.0]);
    }
}