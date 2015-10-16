use super::VendingMachine;
use super::Item;

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
