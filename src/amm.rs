pub struct AMM {
    invariant: u128,
    happens: u128, // reserves
    not: u128,
    funds: u128, // usd invested
}

pub trait Market {
    fn new() -> Self;

    // core methods for supply
    fn split(usd: u128) -> (u128,u128);
    fn merge(amount: u128) -> u128;

    // core method for trade
    fn buy(side: u8, usd: u128, min_amount: u128) -> u128;
    fn sell(side: u8, amount: u128, min_usd: u128) -> u128;
}

impl AMM {
    pub fn reserves(&self) -> (u128, u128) {
        return (self.happens, self.not);
    }

    pub fn funds(&self) -> u128 {
        self.funds
    }

    pub fn invariant(&self) -> u128 {
        self.invariant
    }

    pub fn check_invariant(&self) -> bool {
        return (self.happens * self.not) == self.invariant
    }
}

impl Market for AMM {
    fn new() -> AMM {
        AMM { invariant: 0, happens: 0, not: 0, funds: 0}
    }

    fn merge(amount: u128) -> u128 {
        0
    }

    fn split(usd: u128) -> (u128,u128) {
        (0,0)
    }

    fn buy(side: u8, usd: u128, min_amount: u128) -> u128 {
        0
    }

    fn sell(side: u8, amount: u128, min_usd: u128) -> u128 {
        0
    }
}