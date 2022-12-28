
pub use crate::couleur::Couleur;
use colored::Colorize;
pub struct Carte {
    pub couleur: Couleur,
    pub numero: i8,
}

impl Copy for Carte { }

impl Clone for Carte {
    fn clone(&self) -> Carte {
        *self
    }
}

impl Carte {
    pub fn display(&self) {
        match self.couleur {
            Couleur::Bleu => print!("{}", self.numero.to_string().blue().on_black()),
            Couleur::Jaune => print!("{}", self.numero.to_string().yellow().on_black()),
            Couleur::Rouge => print!("{}", self.numero.to_string().red().on_black()),
            Couleur::Vert => print!("{}", self.numero.to_string().green().on_black()),
        }
        print!(" ");
    }

    pub fn is_playable(&self, carte: & Carte) -> bool {
        if self.couleur == carte.couleur {
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

