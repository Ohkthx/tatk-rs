//! Demonstrates how to initialize and use the various traits.
use tatk::traits::{AsValue, Close, High, Low, Ohlc4, Open, Volume};
use tatk::Num;
use tatk_derive::{Close, High, Low, Open, Volume};

// Holds snapshot data for a product.
#[derive(Open, Close, Low, High, Volume)]
struct Candle {
    open: Num,
    close: Num,
    high: Num,
    low: Num,
    volume: Num,
}

// Add Open, High, Low, Close to the Candle.
impl Ohlc4 for Candle {}

// Add unique AsValue to be passed to indicators.
// Allows for manipulation of data before passing to indicator.
impl AsValue for Candle {
    fn as_value(&self) -> Num {
        self.ohlc4() / self.volume()
    }
}

fn main() {
    // Test candle.
    let c1 = Candle {
        open: 5.0,
        close: 7.0,
        high: 8.0,
        low: 4.0,
        volume: 2.0,
    };

    println!("OHLC Value: {}", c1.ohlc4());
    println!("AsValue Value: {}", c1.as_value());
}
