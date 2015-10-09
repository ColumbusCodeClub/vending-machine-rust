#![allow(dead_code)]
#[cfg(test)]
mod test;

struct VendingMachine {
    coin_return: Vec<u32>,
    coins: Vec<u32>,
}

enum Item { //TODO: we started refactoring products to use enums, but didn't finish
    Pop (i32),
    Chips (i32),
    Candy (i32)
}

impl VendingMachine {
    fn coin_return(&self) -> Vec<u32> {
        self.coin_return.clone()
    }

    fn new() -> Self {
        VendingMachine {
            coin_return: vec![],
            coins: vec![],
        }
    }
    fn display(&self) -> String {
        if self.coins.len() > 0 {
            let x = self.coins.iter().fold(0, |sum, &val|{sum+val});
            format!("{:.2}", x as f32 / 100.0)
        } else {
            "INSERT COIN".to_string()
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
            for coin in self.coins.clone().iter() {
                self.coin_return.push(*coin);
            }
            self.coins.clear();
    }

    fn select_product( &mut self, product:&'static str) -> &'static str{


        self.coins.clear();

         product
    }
}
