//! Demonstrates how to initialize and use the various traits.
use tatk::traits::{AsValue, Close, High, Low, Open, Volume, OHLC4};
use tatk::Num;

// Holds snapshot data for a product.
struct Candle {
    open: Num,
    close: Num,
    high: Num,
    low: Num,
    volume: Num,
}

// Required for OHLC trait.
impl Close for Candle {
    fn close(&self) -> Num {
        self.close
    }
}

// Required for OHLC trait.
impl Open for Candle {
    fn open(&self) -> Num {
        self.open
    }
}

// Required for OHLC trait.
impl Low for Candle {
    fn low(&self) -> Num {
        self.low
    }
}

// Required for OHLC trait.
impl High for Candle {
    fn high(&self) -> Num {
        self.high
    }
}
// Required for the user-defined AsValue trait.
impl Volume for Candle {
    fn volume(&self) -> Num {
        self.volume
    }
}

// Add Open, High, Low, Close to the Candle.
impl OHLC4 for Candle {}

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
