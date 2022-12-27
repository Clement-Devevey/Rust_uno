pub mod deck;
pub mod defausse;
pub mod joueur;
pub mod carte;
pub mod couleur;
use deck::Deck;
use joueur::Joueur;

use text_io::scan;

const NB_OF_CARDS: i8 = 40;

fn main() {
    println!("Création du Deck");
    let mut deck: Deck = Deck { cartes: Vec::with_capacity(NB_OF_CARDS.try_into().unwrap()) };

    deck.build_deck();
    deck.display();

    println!("After suffled");
    deck.shuffle();
    deck.display();


    // Number of players
    print!("Number of player:\n");
    let nb_of_players: i8;
    scan!("{}", nb_of_players);
    println!("Number of player is {nb_of_players}");

    // Ask their names
    let mut joueurs: Vec<Joueur> = Vec::new();
    let mut name: String;

    for i in 0..nb_of_players as usize {
        println!("Player {} what is your name:", i+1);
        scan!("{}", name);
        println!("Your name is {name}");
        joueurs.push(Joueur{main: Vec::new(), nom: name, score: 0});
        joueurs[i].build_main(&mut deck);
    }

    for joueur in joueurs.iter() {
        joueur.display_main();
    }

    println!("Taille du deck après distribution des cartes:{}", deck.cartes.len());

}
