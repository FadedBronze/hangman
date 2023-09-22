mod words;
use rand;

fn get_random_word() -> &'static str {
    let length = words::WORDS.len() as f64;

    let rand_idx: usize = (rand::random::<f64>() * length).floor() as usize;

    words::WORDS[rand_idx]
}

fn get_trimmed_input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line = line.trim().to_owned();
    line
}

fn main() {
    let hangman = [
        "  +---+ \n      |   \n      | \n      | \n      | \n     ===\n",
        "  +---+ \n  O   |   \n      | \n      | \n      | \n     ===\n",
        "  +---+ \n  O   |   \n  |   | \n      | \n      | \n     ===\n",
        "  +---+ \n  O   |   \n/ |   | \n      | \n      | \n     ===\n",
        "  +---+ \n  O   |   \n/ | \\ | \n      | \n      | \n     ===\n",
        "  +---+ \n  O   |   \n/ | \\ | \n /    | \n      | \n     ===\n",
        "  +---+ \n  O   |   \n/ | \\ | \n / \\  | \n      | \n     ===\n",
    ];

    let mut mistakes: usize = 0;
    let word = get_random_word();
    let mut answers: Vec<char> = vec![];

    loop {
        println!("{}", hangman[mistakes]);
        println!(
            "{}",
            word.chars()
                .map(|letter| {
                    if answers.contains(&letter) {
                        letter.to_string()
                    } else {
                        "_".to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join("")
        );

        let answer = get_trimmed_input();

        if answer == word {
            println!("You win! :)");
            break;
        }

        let character = answer.chars().nth(0);

        if let Some(guess) = character {
            if word.contains(guess) {
                answers.push(guess);
            } else {
                mistakes += 1;
            }
        }

        if mistakes > 6 {
            println!("You lose. :(");
            break;
        }
    }

    println!("The word was: {}", word);
}
