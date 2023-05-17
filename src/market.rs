pub struct Market {
    invariant: u128,
    happens: u128, // reserves
    not: u128,
    funds: u128, // usd invested
}

enum Side {
    Happens, Not
}

impl Market {
    pub fn new() -> Market {
        Market { invariant: 0, happens: 0, not: 0, funds: 0 }
    }

    pub fn invariant(&self) -> u128 {
        self.invariant
    }

    pub fn happens(&self) -> u128 {
        self.happens
    }

    pub fn not(&self) -> u128 {
        self.not
    }

    pub fn funds(&self) -> u128 {
        self.funds
    }

    pub fn get_reserves(&self) -> (u128,u128) {
        (self.happens, self.not)
    }

    // usd -> shares (split method)
    pub fn split(&mut self, amount: u128) -> (u128, u128) {
        self.funds += amount;
        (amount, amount)
    }

    // shares -> usd
    pub fn merge(&mut self, shares:u128) -> u128 {
        shares
    }

    fn recalculate_invariant(&mut self) {
        self.invariant = self.happens * self.not;
    }
}