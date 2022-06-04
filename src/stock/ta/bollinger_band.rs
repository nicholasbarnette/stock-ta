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

    let mut i = PERIOD;
    while {
        let ma = sma::run(prices[i-PERIOD..i].to_vec(), PERIOD);
        let ma = match ma.get(0) {
            Some(&v) => v,
            None => panic!("Error calculating SMA.")
        };
        let std = std_dev(&prices);
        let bolu = ma + (2.0 * std);
        let bold = ma - (2.0 * std);
        bbs.push((bold, ma, bolu));
        i += 1;
        i <= prices.len()
    } {}
    // for i in PERIOD..prices.len() {
    //     let ma = sma::run(prices[i-PERIOD..i].to_vec(), PERIOD);
    //     let ma = match ma.get(0) {
    //         Some(&v) => v,
    //         None => panic!("Error calculating SMA.")
    //     };
    //     let std = std_dev(&prices);
    //     let bolu = ma + (2.0 * std);
    //     let bold = ma - (2.0 * std);
    //     bbs.push((bold, ma, bolu));
    // }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_simple() {
        let prices = vec![
            10.0, 10.0, 15.0, 20.0, 20.0,
            10.0, 10.0, 15.0, 20.0, 20.0,
            10.0, 10.0, 15.0, 20.0, 20.0,
            10.0, 10.0, 15.0, 20.0, 20.0
        ];
        assert_eq!(run(prices), vec![(6.055728, 15.0, 23.944271)]);
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
        ];
        assert_eq!(
            run(prices),
            vec![
                (-5.719162, 15.6, 36.919163), (-5.6191626, 15.7, 37.01916),
                (-5.719162, 15.6, 36.919163), (-5.469162, 15.85, 37.169163),
                (-4.3691616, 16.95, 38.269165), (-3.5691624, 17.75, 39.06916),
                (-3.6191616, 17.7, 39.019165), (-3.0691624, 18.25, 39.56916),
                (-3.9191628, 17.4, 38.719162), (-5.8191624, 15.5, 36.81916),
                (-7.269162, 14.05, 35.369164)
            ]
        );
    }

    #[test]
    #[should_panic(expected = "Not enough entries to calculate bollinger bands. Received 1, but required 20.")]
    fn test_run_not_enough_elements() {
        run(vec![10.0]);
    }
}