use std::io;
use std::io::Write;

enum FaceCard {
    Jack,
    Queen,
    King,
    Ace,
    Null,
}

struct Card {
    face: FaceCard,
    value: u8,
}

impl Card {
    fn value(&self) -> &u8 {
        &self.value
    }

    fn value_update(&mut self, newval: u8) {
        let value = newval;
    }

    fn face(&self) -> &FaceCard {
        &self.face
    }
}

fn face_value(card: &Card) -> String {
    match card.face() {
        FaceCard::Jack => String::from("Jack"),
        FaceCard::Queen => String::from("Queen"),
        FaceCard::King => String::from("King"),
        FaceCard::Null => format!("{}", card.value()),
        FaceCard::Ace => format!("{}", card.value()),
    }
}
/*
fn determine_ace(cardcollection: &CardCollection) -> u8 {

    if cardcollection.on_hand
}
*/
struct CardCollection {
    on_hand: Vec<Card>,
}

fn main() {
    let deck = generate_deck(52);
    
    println!("Get as close to 21 as you can without exceeding!");

    loop {
        print!("Please input a command: ");
        io::stdout().flush().unwrap();

        let mut command = String::new();

        io::stdin().read_line(&mut command)
            .expect("Failed to read line");
        
        match command.as_str().trim() {
            "hand" => print_hand(&deck.on_hand),
            "exit" => std::process::exit(1),
            _ => println!("Not a command"),
        }
    } 
}

fn generate_deck(count: u8) -> CardCollection {
    let mut temp_deck = CardCollection{ on_hand: Vec::new()};

    'outer: loop {
        loop {
            temp_deck.on_hand.push(
                Card { 
                    face:
                     FaceCard::King,
                     value: *&count
                }
            );

            break 'outer;
        }
    }
    return temp_deck
}

fn print_hand(hand: &Vec<Card>) {
    print!("Hand Contents: ");
    io::stdout().flush().unwrap();
    hand.iter().for_each(|card| print!("{}", face_value(&card)));
    print!("\n");
}

