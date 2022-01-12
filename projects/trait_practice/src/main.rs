use std::fmt;

pub trait CryptoPrint {
    fn summarize(&self) -> String;
}

pub struct Crypto {
    pub name: String,
    pub symbol: String,
    pub price: f64,
    pub circulating_supply: u64,
    pub market_cap: u64,
}

impl fmt::Display for Crypto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "---{}---
Symbol: {}
Price: ${}
Circulating Supply: {}
Market Cap: {}",
            self.name, self.symbol, self.price, self.circulating_supply, self.market_cap
        )
    }
}

impl CryptoPrint for Crypto {
    fn summarize(&self) -> String {
        format!(
            "---{}---
Symbol: {}
Price: ${}
Circulating Supply: {}
Market Cap: {}",
            self.name, self.symbol, self.price, self.circulating_supply, self.market_cap
        )
    }
}

fn main() {
    let moonriver = Crypto {
        name: String::from("Moonriver"),
        symbol: String::from("MOVR"),
        price: 163.00,
        circulating_supply: 8_969_000,
        market_cap: 457_879_256,
    };

    println!("{}", moonriver.summarize());
    println!("{}", moonriver);
}
