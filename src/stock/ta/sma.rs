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
    let mut i = periods;
    while {
        let sum = prices[i-periods..i].iter().sum::<f32>();
        smas.push(sum / periods as f32);
        i += 1;
        i <= prices.len()
    } {}
    return smas;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_simple() {
        let prices = vec![10.0, 10.0, 15.0, 20.0, 20.0];
        assert_eq!(run(prices, 5), vec![15.0]);
    }

    #[test]
    fn test_run_complex() {
        let prices = vec![10.0, 10.0, 15.0, 20.0, 20.0, 10.0, 10.0, 10.0];
        assert_eq!(run(prices, 5), vec![15.0, 15.0, 15.0, 14.0]);
    }

    #[test]
    #[should_panic(expected = "Not enough entries to calculate the SMA. Received 1, but required 5.")]
    fn test_run_not_enough_elements() {
        run(vec![10.0], 5);
    }
}