use crate::carte::{Carte, Couleur};
use crate::defausse::Defausse;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct Deck {
    pub cartes: Vec<Carte>,
}

impl Deck {
    fn push_all_colors (& mut self, value: &String) {
        self.cartes.push(Carte { couleur: Couleur::Bleu, numero: value.clone() });
        self.cartes.push(Carte { couleur: Couleur::Rouge, numero: value.clone() });
        self.cartes.push(Carte { couleur: Couleur::Jaune, numero: value.clone() });
        self.cartes.push(Carte { couleur: Couleur::Vert, numero: value.clone() });  
    }

    pub fn build_deck(& mut self) {
        // Chaque carte numérotée en deux exemplaires sauf le 0
        for _i in 0..2 {
            for j in 1..10 {
                self.push_all_colors(&j.to_string());
            }
        }
        // une carte 0 de chaque couleur
        self.push_all_colors(&"0".to_string());

        // 4 cartes spéciales de chaque (changement couleur, +4, changement de sens)
        for _i in 0..4 {
            self.cartes.push(Carte { couleur: Couleur::Noir, numero: "c".to_string() });
            self.cartes.push(Carte { couleur: Couleur::Noir, numero: "+4".to_string() });
        }

        // 2 "+2" et sens et saute tour de chaque couleur
        for _i in 0..2 {
            // carte +2
            self.push_all_colors(&"+2".to_string());

            //carte sens
            self.push_all_colors(&"s".to_string());

            //carte saute tour
            self.push_all_colors(&"t".to_string());
        }
    }

    pub fn display(&self) {
        for carte in &self.cartes {
            carte.display();
        }
        print!("\n");
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

    pub fn build_from_defausse(&mut self, defausse: &mut Defausse) {
        std::mem::swap(&mut self.cartes, &mut defausse.cartes);
    }
}