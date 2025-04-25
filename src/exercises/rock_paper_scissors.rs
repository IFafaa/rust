use rand::Rng;

#[derive(PartialEq, Debug)]
enum ERPSOptions {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}
#[derive(Debug)]

enum ERPSFinalWinnerOptions {
    PLAYER = 1,
    IA = 2,
    TIE = 3,
}

#[derive(Debug)]
struct Turn {
    player_option: ERPSOptions,
    ia_option: ERPSOptions,
}

impl Turn {
    fn new(player_option: ERPSOptions) -> Turn {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(1..=3);
        let ia_option = match random_number {
            1 => ERPSOptions::ROCK,
            2 => ERPSOptions::PAPER,
            3 => ERPSOptions::SCISSORS,
            _ => unreachable!(),
        };
        Turn {
            player_option,
            ia_option,
        }
    }

    fn who_win(&self) -> ERPSFinalWinnerOptions {
        match (&self.ia_option, &self.player_option) {
            (ERPSOptions::ROCK, ERPSOptions::PAPER)
            | (ERPSOptions::PAPER, ERPSOptions::SCISSORS)
            | (ERPSOptions::SCISSORS, ERPSOptions::ROCK) => ERPSFinalWinnerOptions::PLAYER,
            (ERPSOptions::PAPER, ERPSOptions::ROCK)
            | (ERPSOptions::SCISSORS, ERPSOptions::PAPER)
            | (ERPSOptions::ROCK, ERPSOptions::SCISSORS) => ERPSFinalWinnerOptions::IA,
            _ => ERPSFinalWinnerOptions::TIE,
        }
    }
}

pub fn rock_paper_scissors() {
    println!("In this game, u will play rock/paper/scissors ultil u lose");
    loop {
        println!("Enter your choose");
        println!("rock / paper / scissors");

        let mut player_choice = String::new();
        std::io::stdin()
            .read_line(&mut player_choice)
            .expect("Error enter your choice");

        let player_choice = player_choice.trim().to_lowercase();

        let player_choice: ERPSOptions = match player_choice.as_str() {
            "rock" => ERPSOptions::ROCK,
            "paper" => ERPSOptions::PAPER,
            "scissors" => ERPSOptions::SCISSORS,
            _ => {
                println!("Invalid choice!");
                return;
            }
        };
        let turn = Turn::new(player_choice);

        let ia_option_text = get_text_option(&turn.ia_option);
        let player_option_text = get_text_option(&turn.player_option);
        let winner_text = get_text_winner(turn.who_win());

        println!("You choose {}", player_option_text);
        println!("IA choose {}", ia_option_text);
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("{} vs {}", player_option_text, ia_option_text);
        std::thread::sleep(std::time::Duration::from_secs(1));
        
        println!("Who winner? {:?}", winner_text);
        println!();
        println!();
        println!();
        println!("------------------------------------------------------");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn get_text_option(option: &ERPSOptions) -> String {
    match option {
        ERPSOptions::ROCK => String::from("Rock"),
        ERPSOptions::PAPER => String::from("Paper"),
        ERPSOptions::SCISSORS => String::from("Scissors"),
    }
}

fn get_text_winner(option: ERPSFinalWinnerOptions) -> String {
    match option {
        ERPSFinalWinnerOptions::IA => String::from("IA"),
        ERPSFinalWinnerOptions::PLAYER => String::from("PLAYER"),
        ERPSFinalWinnerOptions::TIE => String::from("TIE"),
    }
}
