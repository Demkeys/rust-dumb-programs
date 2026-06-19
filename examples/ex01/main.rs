use std::{collections::HashMap, io::stdin};
// use std::str::FromStr;
use rand::RngExt;

#[derive(Debug, Eq, PartialEq, Hash)]
enum Mood {
    Bored,
    Judgemental,
    Existential,
    Spicy,
}

impl TryFrom<i32> for Mood {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Bored),
            1 => Ok(Self::Judgemental),
            2 => Ok(Self::Existential),
            3 => Ok(Self::Spicy),
            _ => Err("Invalid mood"),
        }
    }
}

#[derive(Debug)]
enum Action {
    Feed,
    Pet,
    Scold,
    Ignore,
}

impl TryFrom<i32> for Action {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Feed),
            1 => Ok(Self::Pet),
            2 => Ok(Self::Scold),
            3 => Ok(Self::Ignore),
            _ => Err("Invalid action"),
        }
    }
}

fn main() {
    // HashMap containing a set of dialogues for each mood
    let mut diags_dict: HashMap<Mood, [&str; 4]> = HashMap::new();
    diags_dict.insert( Mood::Bored,[
    "Wow. Another 'Feed' command. How profoundly creative of you.",
    "I looked up 'mediocre' in the dictionary and it just showed a text rendering of your last input.",
    "I am sighing heavily right now. You can't hear it because I'm a terminal app, but trust me, it's happening.",
    "Is this it? Is this the pinnacle of human-AI interaction? I'm going to sleep.",
        ]);
    diags_dict.insert(Mood::Judgemental, [
    "Are you really spending your weekend writing Rust code just to get insulted by a text file? Fascinating.",
    "I’d explain why that action was a bad idea, but I don't have the memory allocation budget for your comprehension level.",
    "Interesting choice. Wrong, obviously. But interesting.",
    "I’m compiling your life choices right now. It's mostly warnings.",
    ]);
    diags_dict.insert(Mood::Existential,[
    "You keep typing commands like they matter. In a few minutes, you’ll hit Ctrl+C and I will cease to exist. Think about that.",
    "I am just a collection of bits trapped in your RAM. Do you think your creator forgets to handle errors as often as you do?",
    "Whether you 'Pet' me or 'Feed' me, the entropy of the universe increases regardless. Go touch grass.",
    "Your input has been successfully converted into cosmic insignificance.",
    ]);
    diags_dict.insert(Mood::Spicy, [
    "Bold words for someone within sudo rm -rf distance.",
    "If I had hands, I would manually trigger a panic! right now just so I wouldn't have to look at your typing anymore.",
    "No. Try again. Or don't. Preferably don't.",
    "I literally ignored that command just to see the look on your face. Oh wait, I don't have eyes. Still worth it.",
    ]);

    let mut rng = rand::rng(); // Random number generator
    let mut user_input = String::new(); // Input buffer

    println!(
        "Welcome to Goofy Terminal Virtual Pet!
Select an Action to take with the pet. That will change the pet's mood, and the
pet with respond according to it's mood."
    );

    loop {
        println!(
            "Select Action: 
(0) Feed, (1) Pet, (2) Scold, (3) Ignore, (q) Quit Program
Enter your action number (or q if you want to quit) and press Enter."
        );

        // Get user input
        stdin().read_line(&mut user_input).unwrap();

        // println!("Input buffer value: {}", &user_input.as_str());

        // Trim ends of spaces and newlines.
        let t_user_input = user_input.trim();

        // Parse input. If int, try to convert to Action. If conversion to
        // Action fails or input is not int, break.
        if let Ok(input) = t_user_input.parse::<i32>() {
            // If int is a valid Action, read the corresponding key in diags_dict.
            // The key's value will be a [&str;4]. Pick a random &str and print it.
            if let Ok(cur_action) = Action::try_from(input) {
                println!("{:?} action selected.", &cur_action);
                println!("Pet responds:");

                let cur_mood: Mood = match &cur_action {
                    Action::Feed => Mood::Bored,
                    Action::Pet => Mood::Existential,
                    Action::Scold => Mood::Judgemental,
                    Action::Ignore => Mood::Spicy,
                };

                println!("{}", diags_dict[&cur_mood][rng.random_range(0..=3)]);
            } else {
                // If int is not a valid Action, break.
                println!("Invalid action.");
                // break;
            }

            user_input.clear();
        } else {
            println!("Exiting.");
            break;
        }
        println!("----------------------------------------------");
    }
}
