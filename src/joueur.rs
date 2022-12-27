use crate::carte::Carte;
use super::Deck;

pub struct Joueur {
    pub main: Vec<Carte>,
    pub score: i32,
    pub nom: String,
}

impl Joueur {
    pub fn build_main(&mut self, deck: &mut Deck) {
        if !self.main.is_empty() {
            self.main.clear();
        }

        for _i in 0..7 {
            self.main.push(deck.draw_card());
        }
    }

    pub fn display_main(& self) {
        println!("{} here are your cards:", self.nom);
        for c in self.main.iter() {
            c.display();
        }
    }

    pub fn display_score(& self) {
        println!("Score of player {} is {}", self.nom, self.score);
    }
}