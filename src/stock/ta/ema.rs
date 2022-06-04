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
    // Use the SMA as its first `ema_prev`
    let mut ema_prev = match sma::run(prices[0..periods].to_vec(), periods).pop() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_simple() {
        let prices = vec![10.0, 10.0, 15.0, 20.0, 20.0, 10.0];
        assert_eq!(run(prices, 5), vec![13.333332]);
    }

    #[test]
    fn test_run_complex() {
        let prices = vec![10.0, 10.0, 15.0, 20.0, 20.0, 10.0, 10.0, 10.0];
        assert_eq!(run(prices, 5), vec![13.333332, 12.222221, 11.48148]);
    }

    #[test]
    fn test_run_random() {
        let prices = vec![5.0, 10.0, 11.0, 6.0, 5.0, 42.0, 33.0, 1.0];
        assert_eq!(run(prices, 5), vec![18.933332, 23.622221, 16.08148]);
    }

    #[test]
    #[should_panic(expected = "Not enough entries to calculate the EMA. Received 5, but required 6 (periods+1).")]
    fn test_run_not_enough_elements() {
        let prices = vec![10.0, 10.0, 15.0, 20.0, 20.0];
        run(prices, 5);
    }
}