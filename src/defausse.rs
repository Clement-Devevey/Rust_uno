use crate::carte::Carte;
pub struct Defausse {
    pub cartes: Vec<Carte>,
}

impl Defausse {
    pub fn ajouter_carte(&mut self, carte: Carte) {
        self.cartes.push(carte);
    }
}