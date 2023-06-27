pub fn explain() {
    println!("To make integration tests, we need to make a new directory called tests next to src (not inside)");
    println!("Go there now, we'll test some functionality")
}

//Some functionality (?)
pub enum TurnstileState { Open, Closed { money_left_to_open: f64 }, }
pub struct Turnstile {
    state: TurnstileState,
    current_balance: f64,
    money_per_turn: f64,
}

impl Turnstile {
    pub fn new(money_per_turn: f64) -> Self {
        Turnstile { 
            state: TurnstileState::Closed { money_left_to_open: money_per_turn }, 
            current_balance: 0.0, 
            money_per_turn,
        }
    }

    pub fn get_money_to_open(&self) -> f64 {
        self.money_per_turn
    }

    pub fn pay(&mut self, money: f64) -> (bool, String) {
        if let TurnstileState::Closed {money_left_to_open} = self.state {
            if money >= money_left_to_open {
                self.state = TurnstileState::Open;
                (true, format!("Thank you for your service. Your change is ${:.2}.", money - money_left_to_open))
            } else {
                (false, String::from("You don't have enough money."))
            }
        } else {
            (true, String::from("The turnstile is already open, please enter."))
        }
    }

    pub fn enter(&mut self) -> (bool, String) {
        if let TurnstileState::Open = self.state {
            self.state = TurnstileState::Closed { money_left_to_open: self.money_per_turn };
            (true, String::from("Enjoy the event."))
        } else {
            (false, format!("Please pay ${:.2} to open the turnstile.", self.money_per_turn))
        }
    }
}