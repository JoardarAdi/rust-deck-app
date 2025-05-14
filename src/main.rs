use rand::{rng, seq::SliceRandom};

#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        // making it an array as the size will never change
        let suits = ["Diamonds", "Spades", "Clubs", "Hearts"];
        let values = ["Ace", "Two", "Three", "Four", "Five"];

        let mut cards = vec![];
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // let deck = Deck { cards: Vec::new() };

        //this is called Implicit Return
        //in rust, the last line of a func is always returned provided it does not have semicolon
        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        //implicit return
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();

    // deck.shuffle();

    // TODO: enable error handling
    let cards = deck.deal(3);

    println!("Heres your hand: {:#?}", cards);
    println!("Heres your deck: {:#?}", deck);
}
