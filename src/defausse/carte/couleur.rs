use std::fmt;

pub enum Couleur{
    Bleu,
    Jaune,
    Rouge,
    Vert,
}

impl fmt::Display for Couleur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Couleur::Bleu => write!(f, "Bleu"),
            Couleur::Jaune => write!(f, "Jaune"),
            Couleur::Rouge => write!(f, "Rouge"),
            Couleur::Vert => write!(f, "Vert"),
        }
    }
}