use crate::card::{Card, Name};

mod card;

fn main() {
    let mut cards: Vec<Card> = vec![];
    for i in 2..=10 {
        let new_card = Card::new(Name::Number(i), 'H');
        cards.push(new_card);
    }

    for c in ['J', 'Q', 'K', 'A'] {
        let new_card = Card::new(Name::Face(c), 'H');
        cards.push(new_card);
    }

    for card in cards {
        println!("{}", card);
    }
}
