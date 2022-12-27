
pub use crate::couleur::Couleur;

pub struct Carte {
    pub couleur: Couleur,
    pub numero: i8,
}

impl Carte {
    pub fn display(&self) {
        print!("Valeur: {} - Couleur: {}\n", self.numero, self.couleur);
    }
}

