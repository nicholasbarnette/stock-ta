use std::{path::Path};

mod stock;
// mod json;

fn main() {
    println!("Hello, world!");

    let mut s = stock::Stock::new("aapl");
    match s.load_data(Path::new("./src/assets/contemporary/aapl.csv")) {Ok(b) => b, Err(error) => panic!("{}", error)};
    match s.load_historical_data(Path::new("./src/assets/historical/aapl.csv")) {Ok(b) => b, Err(error) => panic!("{}", error)};
    s.backtest();

    let data: Vec<&stock::historical::HistoricalData> = s.query_historical_data(
        &stock::datetime::DateTime::new("1/1/2021 00:00:00"),
        &stock::datetime::DateTime::new("12/31/2021 00:00:00")
    );
    let prices: Vec<f32> = data.iter().map(|el| el.get_close()).collect();

    const NUM_DAYS: usize = 50;
    let smas = stock::ta::sma::run(prices, NUM_DAYS);
    println!("smas {:?}\n\n", smas);

    let prices: Vec<f32> = data.iter().map(|el| el.get_close()).collect();
    let emas = stock::ta::ema::run(prices, NUM_DAYS);
    println!("emas {:?}\n\n", emas);

    let prices: Vec<f32> = data.iter().map(|el| el.get_close()).collect();
    let macds = stock::ta::macd::run(prices);
    println!("macds {:?}\n\n", macds);

    let prices: Vec<f32> = data.iter().map(|el| el.get_close()).collect();
    let rsis = stock::ta::rsi::run(prices);
    println!("rsis {:?}\n\n", rsis);

    
    let prices: Vec<f32> = data.iter().map(|el| el.get_close()).collect();
    let volume: Vec<u32> = data.iter().map(|el| el.get_volume()).collect();
    let obvs = stock::ta::obv::run(prices, volume);
    println!("obvs {:?}\n\n", obvs);


    // let s = json::sanitize::sanitize("{
    //     \"test\": \"testing 1\",
    //     \"test2\": \"testing 2\",
    //     \"test3\": \"testing 3 this is longer\",
    //     \"test4\": 100,
    //     \"test5\": true,
    //     \"test6\": { \"test\": \"testing 1\" },
    //     \"test7\": [{ \"test\": \"testing 1\" }]
    //   }");
    //   println!("{}", s);
    //   let s = json::pretty_print::pretty_print(&s);
    //   println!("{}", s);


    // let t = json::serialize::serialize("{\"property1\": \"value 1\",\"property2\": \"value 2\"}");
    // println!("{:?}", t);
    // println!("{:?}", t.find("property1"));

    // let t = json::serialize::serialize("{
    //   \"test\": \"testing 1\",
    //   \"test2\": \"testing 2\",
    //   \"test3\": \"testing 3 this is longer\",
    //   \"test4\": 100,
    //   \"test5\": true,
    //   \"test6\": { \"test\": \"testing 1\" },
    //   \"test7\": [{ \"test\": \"testing 1\" }]
    // }");
    // println!("{:?}", t);
    // let tmp = match t.find("test6").unwrap() {
    //   json::serialize::JSONObject::Number(n) => n,
    //   json::serialize::JSONObject::String(n) => n,
    // };
    // println!("{:?}", tmp);
}
