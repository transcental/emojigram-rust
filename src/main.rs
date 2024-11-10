use std::collections::HashMap;
use std::io::Write;

use rand::{Rng, seq::SliceRandom};

fn main() {
    let sentences: Vec<String> = vec![
        String::from("Amber is silly and likes building slack bots"),
        String::from("Dinobox confessed to Orpheus and got rejected"),
        String::from("Aarya is the best head shark"),
        String::from("Sinerider is a game about love and graphing, built by teenagers"),
        String::from("Heidi Hakkuun confessed to Orpheus and got accepted"),
        String::from("The Low Skies pushed Earth into the High Seas to make the sea bed"),
        String::from("Hackpad is a blot in the horizon"),
        String::from("The wee woo team always puts out the fires in Slack"),
        String::from("After an apocalypse comes a revoolution"),
        String::from("The Boreal Express went chuga chuga in Vancouver"),
        String::from("HCB is a fiscal sponsorship program for Hack Clubbers"),
        String::from("Anxiety Wolf has some interesting thoughts on the world"),
    ];
    let mut emojis: Vec<char> = vec![
        '🤯', '😭', '😏', '😋', '🙀', '🍪', '👿', '😢', '😇', '🥳', '🥺', '🥶', '🤮', '🤧', '🤑',
        '🤠', '🤢', '🤡', '👺', '😻', '👽', '🤖', '👾', '👻'
    ];
    println!("Welcome to my emoji puzzle. You will get a random puzzle about Hack Club or a Hack Clubber that you need to work out. This is like a cryptogram, but instead of replacing letters with letters, letters are replaced with emojis!\n\n");

    let sentence: &String = sentences.choose(&mut rand::thread_rng()).unwrap();
    let mut charmap: HashMap<String, String> = HashMap::new();

    for char in sentence.chars() {
        if char.is_alphabetic() & charmap.get(&char.to_string()).is_none() {
            let e: &char = emojis.choose(&mut rand::thread_rng()).unwrap();
            charmap.insert(char.to_string(), e.to_string());
            let e_index: usize = emojis.iter().position(|&r| r == *e).unwrap();
            emojis.remove(e_index);
        }
    }

    print!("What difficulty would you like? (1-10)");
    print!("\n>>> ");
    let _ = std::io::stdout().flush();
    let mut difficulty = String::new();
    std::io::stdin().read_line(&mut difficulty).unwrap();
    let difficulty: usize = difficulty.trim().parse().unwrap();
    if difficulty < 1 || difficulty > 10 {
        println!("Invalid difficulty level. Please enter a number between 1 and 10.");
        return;
    }

    let emojified_sentence: String = sentence.chars().map(|c| {
        if c.is_alphabetic() {
            charmap.get(&c.to_string()).unwrap().to_string()
        } else {
            c.to_string()
        }
    }).collect();

    let mut rng = rand::thread_rng();
    let parsable_sentence = sentence.chars().enumerate().map(|(_, c)| {
        if c.is_alphabetic() {
            let rand = rng.gen_range(0..difficulty + 1);
            if rand == 0 {
                c.to_string()
            } else {
                charmap.get(&c.to_string()).unwrap().to_string()
            }
        } else {
            c.to_string()
        }
    }).collect::<String>();
    
    let guesses: usize = 3;
    for guess in 0..guesses {
        println!("{}\n{}", emojified_sentence, parsable_sentence);
        println!("\n");
        println!("What do you think the sentence is? ({}/{} attempts reamaining):", guesses - guess, guesses);
        let mut sentence_guess = String::new();
        std::io::stdin().read_line(&mut sentence_guess).unwrap();
        if sentence_guess.trim() == sentence {
            return println!("You got it right with {} attempts left!\n\nNice work, I hope you liked this little demo. There may be more to come soon...", guesses - guess);
        } else {
            println!("Oops, you got it wrong!");
        }
    }
    println!("You ran out of guesses! The sentence was: {}", sentence);

}
