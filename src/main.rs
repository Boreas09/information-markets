use crate::amm::AMM;
use crate::amm::Market;

mod amm;
fn main() {
    
    println!("Hello, world!");
    let market: AMM = Market::new();
    println!("Value: {}", market.funds());
}
