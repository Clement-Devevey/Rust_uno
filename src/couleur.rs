use std::fmt;
#[derive(PartialEq)] //for == operator
pub enum Couleur{
    Bleu,
    Jaune,
    Rouge,
    Vert,
    Noir,
}

impl Copy for Couleur { }

impl Clone for Couleur {
    fn clone(&self) -> Couleur {
        *self
    }
}

impl fmt::Display for Couleur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Couleur::Bleu => write!(f, "Bleu"),
            Couleur::Jaune => write!(f, "Jaune"),
            Couleur::Rouge => write!(f, "Rouge"),
            Couleur::Vert => write!(f, "Vert"),
            Couleur::Noir => write!(f, "Spécial"),
        }
    }
}