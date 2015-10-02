use super::VendingMachine;

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
    vm.insert_coin(25);
    vm.insert_coin(25);
    // vm.press_coin_return();
    assert_eq!(vm.coin_return(), vec![25,25])
}

#[test]
fn it_does_not_accept_pennies() {
    let mut vm = VendingMachine::new();
    vm.insert_coin(1);
    assert_eq!(vm.display(), "INSERT COIN");
    assert_eq!(vm.coin_return(), vec![1]);
}
