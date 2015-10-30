#![allow(dead_code)]
#[cfg(test)]
mod test;

static INSERT_COIN_MSG: &'static str = "INSERT COIN";

struct VendingMachine {
    coin_return: Vec<u32>,
    coins: Vec<u32>,
    token_return: Vec<Token>,
    tokens: Vec<Token>,
}

#[derive(PartialEq, Debug)]
enum Item {
    Pop,
    Chips,
    Candy,
}

#[derive(Debug, Clone)]
struct Token {
    diameter: f64,
    width: f64,
    weight: f64,
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        (self.diameter - other.diameter).abs() <= 0.0001
    }
}

impl VendingMachine {
    fn coin_return(&self) -> Vec<u32> {
        self.coin_return.clone()
    }

    fn token_return(&self) -> Vec<Token> {
        self.token_return.clone()
    }

    fn new() -> Self {
        VendingMachine {
            coin_return: vec![],
            coins: vec![],
            token_return: vec![],
            tokens: vec![],
        }
    }
    fn display(&self) -> String {
        if self.coins.len() > 0 {
            let x = self.coins.iter().fold(0, |sum, &val|{sum+val});
            format!("{:.2}", x as f32 / 100.0)
        } else {
            INSERT_COIN_MSG.to_string()
        }
    }

    fn insert_token(&mut self, token: Token) {
        if token.weight < 2.3 {
            self.insert_coin(10)
        } else  if token.weight > 5.0 {
            self.insert_coin(25)
        } else if token.weight > 2.7 {
            self.insert_coin(5)
        } else{
            self.token_return.push(token)
        }
    }

    fn insert_coin(&mut self, coin: u32) {
        if coin != 1 {
            self.coins.push(coin)
        }
        else {
            self.coin_return.push(coin)
        }
    }

    fn press_coin_return(&mut self) {
            for &coin in self.coins.iter() {
                self.coin_return.push(coin);
            }
            self.coins.clear();
    }

    #[allow(unused_variables)]
    fn select_product(&mut self, location_id: u32) -> Item {
        self.coins.clear();
        Item::Pop
    }
}
