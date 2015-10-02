#![allow(dead_code)]
#[cfg(test)]
mod test;

struct VendingMachine {
    coin: u32,
}

impl VendingMachine {
    fn coin_return(&self)->Vec<u32>{
        if self.coin > 0 {
            vec![25,25]
        }
        else {
            vec![1]
        }
    }

    fn new() -> Self {
        VendingMachine {
            coin: 0
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
        }
    }

    // fn press_coin_return(&mut self) {
    // }
}
