fn main() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(s) => {
                println!("{:?}", s);
                25
            }
        }
    }
    value_in_cents(Coin::Quarter(UsState::Alabama));

    fn get_val(val: Option<i32>) -> i32 {
        match val {
            Some(i) => i,
            // None => 0,
            _ => 0
        }
    }
    let raw_val = 1;
    let val = Some(raw_val);
    println!("{0}", 1 + get_val(val) + get_val(None));
}
