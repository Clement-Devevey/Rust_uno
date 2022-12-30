use crate::carte::Carte;
use crate::deck::Deck;

pub struct Joueur {
    pub main: Vec<Carte>,
    pub score: i32,
    pub nom: String,
}

impl Joueur {
    pub fn display_main(& self) {
        println!("{} here are your cards:", self.nom);
        for c in self.main.iter() {
            c.display();
        }
        println!("\n--------------------------");
    }

    pub fn display_score(& self) {
        println!("Score of player {} is {}", self.nom, self.score);
    }

    pub fn choice_is_playable(& self, choix: u8, last_card_played: &Carte) -> bool {
        if self.main[choix as usize].is_playable(last_card_played) {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn get_nb_card(& self) -> usize {
        self.main.len()
    }
}

pub fn build_main(deck: &mut Deck) -> Vec<Carte> {
    let mut main: Vec<Carte> = Vec::new();
    for _i in 0..7 {
        main.push(deck.draw_card());
    }
    return main;
}