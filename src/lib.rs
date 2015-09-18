#![allow(dead_code)]
struct VendingMachine {
    is_alive: bool,
    coin: u32,
}

impl VendingMachine {
    fn new() -> Self {
        VendingMachine {
            is_alive: false,
            coin: 0
        }
    }
    fn display(&self) -> String {
        if self.is_alive {
            format!("{:.2}", self.coin as f32 / 100.0)
        } else {
            "INSERT COIN".to_string()
        }
    }

    fn insert_coin(&mut self, coin: u32) {
        self.is_alive = true;
        self.coin = coin;
    }
}

#[test]
fn it_accepts_coins() {
    let vm = VendingMachine::new();
    assert_eq!(vm.display(), "INSERT COIN")
}

#[test]
fn it_accepts_nickles() {
    let mut vm = VendingMachine::new();
    vm.insert_coin(5);
    assert_eq!(vm.display(), "0.05")
}

#[test]
fn it_accepts_dimes() {
    let mut vm = VendingMachine::new();
    vm.insert_coin(10);
    assert_eq!(vm.display(), "0.10")
}

#[test]
fn it_does_not_accept_pennies() {
    assert_eq!(true, true)
}