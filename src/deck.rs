pub mod carte;
use carte::{Carte, Couleur};

use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct Deck {
    pub cartes: Vec<Carte>,
}

impl Deck {
    pub fn build_deck(& mut self) {
        for j in 0..10 {
            self.cartes.push(Carte { couleur: Couleur::Bleu, numero: j });
            self.cartes.push(Carte { couleur: Couleur::Rouge, numero: j });
            self.cartes.push(Carte { couleur: Couleur::Jaune, numero: j });
            self.cartes.push(Carte { couleur: Couleur::Vert, numero: j });
        }
    }

    pub fn display(&self) {
        for carte in &self.cartes {
            carte.display();
        }
    }

    pub fn shuffle(& mut self) {
        self.cartes.shuffle(&mut thread_rng());
    }

    pub fn draw_card(& mut self) -> Carte {
        match self.cartes.pop() {
            Some(carte) => carte,
            None => panic!("No more card in deck"),
        }
    }
}