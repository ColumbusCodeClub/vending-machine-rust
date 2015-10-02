#![allow(dead_code)]
#[cfg(test)]
mod test;

struct VendingMachine {
    coin: u32,
    coin_return: Vec<u32>,
    coins: Vec<u32>,
}

impl VendingMachine {
    fn coin_return(&self) -> Vec<u32> {
        self.coin_return.clone()
    }

    fn new() -> Self {
        VendingMachine {
            coin: 0,
            coin_return: vec![],
            coins: vec![],
        }
    }
    fn display(&self) -> String {
        if self.coin != 0 {
            format!("{:.2}", self.coin as f32 / 100.0)
        } else {
            "INSERT COIN".to_string()
        }
    }

    fn insert_coin(&mut self, coin: u32) {
        if coin != 1 {
            self.coin += coin;
            self.coins.push(coin)
        }
        else {
            self.coin_return.push(coin)
        }
    }

    fn press_coin_return(&mut self) {
        if self.coin > 0 {
            for coin in self.coins.clone().iter() {
                self.coin_return.push(*coin);
            }
            self.coin = 0
        }
    }
}
