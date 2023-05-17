use crate::market::Market;

mod market;
fn main() {
    
    println!("Hello, world!");
    let market: Market = Market::new();
    println!("Value: {}", market.invariant());
}
