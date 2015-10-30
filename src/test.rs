use super::VendingMachine;
use super::Token;
use super::Item;
use super::INSERT_COIN_MSG;

const DIME: Token = Token {
    diameter: 17.9,
    weight: 2.268,
    width: 1.35,
};
const NICKLE: Token = Token {
    diameter: 21.21,
    weight: 5.0,
    width: 1.95,
};
const PENNY: Token = Token {
    diameter: 19.05,
    weight: 2.5,
    width: 1.52,
};
const QUARTER: Token = Token {
    diameter: 24.26,
    weight: 5.670,
    width: 1.75,
};


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
fn it_accepts_quarters() {
    let mut vm = VendingMachine::new();
    vm.insert_coin(25);
    assert_eq!(vm.display(), "0.25")
}

#[test]
fn it_accepts_two_quarters() {
    let mut vm = VendingMachine::new();
    vm.insert_coin(25);
    vm.insert_coin(25);
    assert_eq!(vm.display(), "0.50")
}

#[test]
fn it_should_return_coins_when_coin_return_is_pressed() {
    let mut vm = VendingMachine::new();
    vm.insert_coin(1);
    vm.insert_coin(25);
    vm.insert_coin(25);
    assert_eq!(vm.coin_return(), vec![1]);
    vm.press_coin_return();
    assert_eq!(vm.coin_return(), vec![1, 25, 25]);
    assert_eq!(vm.display(), "INSERT COIN");

    vm.insert_coin(5);
    vm.press_coin_return();
    assert_eq!(vm.coin_return(), vec![1, 25, 25, 5]);
}


#[test]
fn it_does_not_accept_pennies() {
    let mut vm = VendingMachine::new();
    vm.insert_coin(1);
    assert_eq!(vm.display(), "INSERT COIN");
    assert_eq!(vm.coin_return(), vec![1]);
}

#[test]
fn it_adds_a_nickle_and_dime_and_returns_them() {
    let mut vm = VendingMachine::new();
    vm.insert_coin(5);
    vm.insert_coin(10);
    assert_eq!(vm.display(), "0.15");
    vm.press_coin_return();
    assert_eq!(vm.coin_return(), vec![5, 10]);
}

#[test]
fn it_should_vend_when_enough_money_is_inserted(){
    let mut vm = VendingMachine::new();
    vm.insert_coin(25);
    vm.insert_coin(25);
    vm.insert_coin(25);
    vm.insert_coin(25);
    let item = vm.select_product(2);
    assert_eq!(vm.coin_return(), vec![]);
    assert_eq!(item, Item::Pop);
}

#[test]
fn insert_token_accepts_nickle() {
    let mut vm = VendingMachine::new();
    vm.insert_token(NICKLE);
    assert_eq!(vm.display(), "0.05");
}

#[test]
fn insert_token_accepts_dime() {
    let mut vm = VendingMachine::new();
    vm.insert_token(DIME);
    assert_eq!(vm.display(), "0.10");
}


#[test]
fn insert_token_rejects_penny() {
    let mut vm = VendingMachine::new();
    vm.insert_token(PENNY);
    assert_eq!(vm.display(), INSERT_COIN_MSG);
    assert_eq!(vm.token_return(), vec![PENNY]);
}

#[test]
fn insert_token_accepts_quarter(){
    let mut vm = VendingMachine::new();
    vm.insert_token(QUARTER);
    assert_eq!(vm.display(), "0.25");

}

#[test]
fn two_of_exactly_the_same_token_should_be_equal() {
    let t1 = PENNY;
    let t2 = PENNY;
    assert_eq!(t1, t2);
}

#[test]
fn two_approximately_same_diameter_tokens_should_be_equal() {
    let mut t1 = PENNY;
    t1.diameter += 0.0001;
    let t2 = PENNY;
    assert_eq!(t1, t2);
}
