enum IpAddr {
    V4(String)
    V6(String)
}

enum IpAddrVariant {
    V4(u8, u8, u8, u8)
    V6(String)
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum UsState {
    California,
    Nevada,
    Alabama,
}

enum CoinWithState {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl IpAddrVariant {
    fn validate(&self) {
    // body method
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents_with_log(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky coin!");
            1
        },
        Coin::Nickel => 5,
        ...
    }
}

fn value_in_cents_state(coin: CoinWithState) -> u8 {
    match coin {
        CoinWithState::Penny => 1,
        CoinWithState::Nickel => 5,
        CoinWithState::Dime => 10,
        CoinWithState::QuarterO(state) => {
            println!("State quarterfrom {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) => Option<i32> {
    match x {
        None => None
        Some(i) => Some(x + 1)
    }
}

fn enum_game() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // This will call the function ignoring the value
        // other => move(other)
        // _ => (), This case will do nothing at all
    }
    
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
    fn move(m: i32) {}
}

fn control_flow_with_if_let(max: i32) {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        println!("Not the maximum yet");
    }
}