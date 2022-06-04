use crate::stock::Stock;

pub fn run(stock: &Stock) {
    println!("{}", stock.get_ticker());
}