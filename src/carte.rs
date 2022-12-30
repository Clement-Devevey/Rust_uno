
pub use crate::couleur::Couleur;
use colored::Colorize;
pub struct Carte {
    pub couleur: Couleur,
    pub numero: String,
}

impl Clone for Carte {
    fn clone(&self) -> Carte {
        let couleur = self.couleur;
        let numero = self.numero.clone();
        return Carte { couleur: couleur, numero: numero };
    }
}

impl Carte {
    pub fn display(&self) {
        match self.couleur {
            Couleur::Bleu => print!("{}", self.numero.blue().on_black()),
            Couleur::Jaune => print!("{}", self.numero.yellow().on_black()),
            Couleur::Rouge => print!("{}", self.numero.red().on_black()),
            Couleur::Vert => print!("{}", self.numero.green().on_black()),
            Couleur::Noir => print!("{}", self.numero.white().on_black()),
        }
        print!(" ");
    }

    pub fn is_playable(&self, carte: & Carte) -> bool {
        // Carte noire (spéciale) peut-être posée sur n'importe quelle autre.
        if self.couleur == Couleur::Noir || self.couleur == carte.couleur {
            return true;
        }
        else if self.numero == carte.numero {
            return true;
        }
        else {
            return false;
        }

    }
}

