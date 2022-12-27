pub mod deck;
pub mod defausse;
pub mod joueur;

use deck::Deck;

const NB_OF_CARDS: i8 = 40;

fn main() {
    println!("Création du Deck");
    let mut deck: Deck = Deck { cartes: Vec::with_capacity(NB_OF_CARDS.try_into().unwrap()) };

    deck.build_deck();
    deck.display();

    println!("After suffled");
    deck.shuffle();
    deck.display();

    println!("Draw card");
    deck.draw_card().display();
    println!("Deck after drawing: ");
    deck.display();
}
