use std::io::stdin;
use std::time::{Duration, SystemTime};
use std::time::UNIX_EPOCH;

#[derive(Debug)]

struct PasswordSettings {
    number_characters: u8,
    uppercase_letters: bool,
    numbers: bool,
    special_characters: bool,
}

impl PasswordSettings {
    fn change_number_characters(&mut self) {
        let mut user_input = String::new();
        println!("How many characters do you want your password to have (at least 8)");
        stdin().read_line(&mut user_input).expect("Failed reading User Input");
        match user_input.trim().parse::<u8>() {
            Ok(num) => {
                if num >=8 {
                self.number_characters = num;
                } else {
                    println!("Please at least 8");
                    println!("Try Again!");
                    self.change_number_characters();
                }
            }
            Err(_) => {
                println!("Invalid Input");
                println!("Try Again!");
                self.change_number_characters();
            },
        }
    }

    fn change_uppercase_letters(&mut self) {
        let mut user_input = String::new();
        println!("Should the Password contain uppercase letters?");
        println!("Yes/No");
        match stdin().read_line(&mut user_input) {
            Ok(_) => {
                match user_input.trim().to_lowercase().as_str() {
                    "y" | "yes" => self.uppercase_letters = true,
                    "n" | "no" => self.uppercase_letters = false,
                    _ => {
                            println!("Invalid Input");
                            println!("Try Again!");
                            self.change_uppercase_letters();
                    },
                }
            },
            Err(err) => println!("The following Error occured: {}", err),
        }
    }
    fn change_numbers(&mut self) {
        let mut user_input = String::new();
        println!("Should the Password contain numbers?");
        println!("Yes/No");
        match stdin().read_line(&mut user_input) {
            Ok(_) => {
                match user_input.trim().to_lowercase().as_str() {
                    "y" | "yes" => self.numbers = true,
                    "n" | "no" => self.numbers = false,
                    _ => {
                            println!("Invalid Input");
                            println!("Try Again!");
                            self.change_numbers();
                    },
                }
            },
            Err(err) => println!("The following Error occured: {}", err),
        }
    }
    fn change_special_characters(&mut self) {
        let mut user_input = String::new();
        println!("Should the Password contain special characters?");
        println!("Yes/No");
        match stdin().read_line(&mut user_input) {
            Ok(_) => {
                match user_input.trim().to_lowercase().as_str() {
                    "y" | "yes" => self.special_characters = true,
                    "n" | "no" => self.special_characters = false,
                    _ => {
                            println!("Invalid Input");
                            println!("Try Again!");
                            self.change_special_characters();
                    },
                }
            },
            Err(err) => println!("The following Error occured: {}", err),
        }
    }
    fn configure_settings(&mut self) {
        println!("===Password Generator Settings===");
        self.change_number_characters();
        self.change_uppercase_letters();
        self.change_numbers();
        self.change_special_characters();
        println!("===Configuration Complete===");

    }
}

struct RandomNumberGenerator {
    seed_1: u64,
    seed_2: u64,
    time_in_sec: u32,
}

impl RandomNumberGenerator {
    fn set_time_in_sec(&mut self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.time_in_sec = now as u32;
    }
    fn create_random_number(&mut self, min: u64, max: u64) -> u64 {
        if min >= max {
            panic!("min muss kleiner als max sein!");
        }
        self.set_time_in_sec();
        let range = max - min;
        let product = self.seed_1
            .wrapping_mul(self.seed_2)
            .wrapping_mul(self.time_in_sec as u64);
        let random_value = product % range as u64;
        min + random_value
    }
    fn reset_seeds(&mut self) {
        self.set_time_in_sec();
        self.seed_1 = self.create_random_number(0, 18446744073709551615);
        self.seed_2 = self.create_random_number(0, 18446744073709551615);
    }
}

fn main() {
    let mut password_settings = PasswordSettings{
        number_characters: 12,
        uppercase_letters: true,
        numbers: true,
        special_characters: true,
    };
    let mut random_number_generator = RandomNumberGenerator{
        seed_1: 12847392058471829637,
        seed_2: 5921083746291455983,
        time_in_sec: 0,
    };
    let num = random_number_generator.create_random_number(1, 23);
    print!("{}", num);

}
