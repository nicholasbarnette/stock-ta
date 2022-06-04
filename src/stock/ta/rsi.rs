/// Calculate the relative strength index (RSI) of a `Vec<f32>` of price data.
/// 
/// ### Definition
/// Measures the magnitude of recent price changes to determine whether
/// a security is overbought or oversold.
/// 
/// ### Formula
/// `rsi1 = 100 - (100 / (1 + (ag / al)))`
/// 
/// `rsi2 = 100 - (100 / 1 + [((pag * 13) + cg) / ((pal * 13) + cl)])`
/// 
/// - `ag`: average gain over a period
/// - `al`: average loss over a period
/// - `pag`: previously calculated average gain
/// - `pal`: previously calculated average loss
/// - `cg`: gain from previous period's price (or 0 if a loss)
/// - `cl`: loss from previous period's price (or 0 if a gain)
/// 
/// NOTE: The first calculation of the RSI is calculated by `rs1` above whereas the
/// remaining RSIs are calculated using `rsi2`.
/// 
/// ### Usage
/// When increasing above 70, the RSI signifies that the security is overbought.
/// This implies that the security could be ready for a momentum reversal and
/// selling may occur. Alternatively, when decreasing below 30, the RSI signifies
/// that the security is oversold and could experience a buy up.
/// 
/// If, in an upward trending sequence, the RSI fails to reach 70 multiple times
/// and then breaks below 30, this is a sign of a possible reversal in momentum
/// to the down side. Inversely, if, in a downward trending sequence, the RSI
/// fails to fall below 30 multiple times and subsequently breaks above 70, it
/// might be a sign of a reversal to the up side.
/// 
/// If the RSI shows increasing lows while the price shows decreasing lows, this
/// is an example of bullish divergence and could signify bullish momentum if the
/// RSI breaks above 30. Conversely, if the RSI shows decreasing highs while the
/// price shows increasing highs, this is an example of bearish divergence and
/// could signify bearish momentum if the RSI breaks below 70.
/// 
/// ##### Swing Rejections
/// 
/// ###### Bullish Swing Rejection
/// 1. RSI enters oversold territory (RSI < 30)
/// 2. RSI exits oversold territory (RSI > 30)
/// 3. RSI dips below #2 reading, but stays above 30
/// 4. RSI breaks most recent high (#2)
/// 
/// ###### Bearish Swing Rejection
/// 1. RSI enters overbought territory (RSI > 70)
/// 2. RSI exits oversold territory (RSI < 70)
/// 3. RSI pushes above #2 reading, but stays below 70
/// 4. RSI breaks most recent low (#2)
/// 
/// # Arguments
/// * `prices` - `Vec<f32>` containing prices for a period of time
/// 
/// ### Example
/// ```
/// rsi::run(prices);
/// rsi::run(prices);
/// ```
/// 
/// #### Resources
/// - https://www.investopedia.com/terms/r/rsi.asp
pub fn run(prices: Vec<f32>) -> Vec<f32> {
    const PERIOD: usize = 14;
    if prices.len() < PERIOD+1 { panic!("Not enough entries to calculate the RSI. Received {}, but required {}.", prices.len(), PERIOD+1); }

    // AVG Gain/Loss
    let mut ag: f32 = 0.0;
    let mut al: f32 = 0.0;
    let mut last_price: f32 = 0.0;
    let mut rsis: Vec<f32> = Vec::new();
    for i in 0..PERIOD+1 {
        if i == 0 {
            last_price = match prices.get(0) {
                Some(&v) => v,
                None => 0.0,
            };
            continue;
        }
        let current_price = match prices.get(i) {
            Some(&v) => v,
            None => 0.0,
        };
        if current_price > last_price {
            ag += current_price - last_price;
            al += 0.0;
        } else if current_price < last_price {
            ag += 0.0;
            al += last_price - current_price;
        }
        last_price = current_price;
    }
    ag = ag / PERIOD as f32;
    al = al / PERIOD as f32;
    let rs = ag / al;
    let rsi_1 = 100.0 - (100.0 / (1.0 + rs));
    rsis.push(rsi_1);

    // Find remaining RSIs
    for i in PERIOD+2..prices.len() {
        let current_price = match prices.get(i) {
            Some(&v) => v,
            None => 0.0,
        };
        if current_price > last_price {
            ag = ((ag * 13.0) + current_price) / 14.0;
            al = ((al * 13.0) + 0.0) / 14.0;
        } else if current_price < last_price {
            ag = ((ag * 13.0) + 0.0) / 14.0;
            al = ((al * 13.0) + current_price) / 14.0;
        }
        let rs = ag / al;
        let rsi = 100.0 - (100.0 / (1.0 + rs));
        rsis.push(rsi);
        last_price = current_price;
    }
    return rsis;
}