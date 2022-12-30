use crate::carte::{Carte, Couleur};
use crate::defausse::Defausse;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct Deck {
    pub cartes: Vec<Carte>,
}

impl Deck {
    pub fn build_deck(& mut self) {
        // Chaque carte numérotée en deux exemplaires sauf le 0
        for _i in 0..2 {
            for j in 1..10 {
                self.cartes.push(Carte { couleur: Couleur::Bleu, numero: j.to_string() });
                self.cartes.push(Carte { couleur: Couleur::Rouge, numero: j.to_string() });
                self.cartes.push(Carte { couleur: Couleur::Jaune, numero: j.to_string() });
                self.cartes.push(Carte { couleur: Couleur::Vert, numero: j.to_string() });
            }
        }
        // une carte 0 de chaque couleur
        self.cartes.push(Carte { couleur: Couleur::Bleu, numero: "0".to_string() });
        self.cartes.push(Carte { couleur: Couleur::Rouge, numero: "0".to_string() });
        self.cartes.push(Carte { couleur: Couleur::Jaune, numero: "0".to_string() });
        self.cartes.push(Carte { couleur: Couleur::Vert, numero: "0".to_string() });
        // 4 cartes spéciales de chaque (changement couleur, +4, changement de sens)
        for _i in 0..4 {
            self.cartes.push(Carte { couleur: Couleur::Noir, numero: "c".to_string() });
            self.cartes.push(Carte { couleur: Couleur::Noir, numero: "+4".to_string() });
        }

        // 2 "+2" et sens et saute tour de chaque couleur
        for _i in 0..2 {
            // carte +2
            self.cartes.push(Carte { couleur: Couleur::Bleu, numero: "+2".to_string() });
            self.cartes.push(Carte { couleur: Couleur::Rouge, numero: "+2".to_string() });
            self.cartes.push(Carte { couleur: Couleur::Jaune, numero: "+2".to_string() });
            self.cartes.push(Carte { couleur: Couleur::Vert, numero: "+2".to_string() });
            //carte sens
            self.cartes.push(Carte { couleur: Couleur::Bleu, numero: "s".to_string() });
            self.cartes.push(Carte { couleur: Couleur::Rouge, numero: "s".to_string() });
            self.cartes.push(Carte { couleur: Couleur::Jaune, numero: "s".to_string() });
            self.cartes.push(Carte { couleur: Couleur::Vert, numero: "s".to_string() });
            //carte saute tour
            self.cartes.push(Carte { couleur: Couleur::Bleu, numero: "t".to_string() });
            self.cartes.push(Carte { couleur: Couleur::Rouge, numero: "t".to_string() });
            self.cartes.push(Carte { couleur: Couleur::Jaune, numero: "t".to_string() });
            self.cartes.push(Carte { couleur: Couleur::Vert, numero: "t".to_string() });
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