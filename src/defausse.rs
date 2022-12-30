use crate::carte::Carte;
pub struct Defausse {
    pub cartes: Vec<Carte>,
}

impl Defausse {
    pub fn ajouter_carte(&mut self, carte: &Carte) {
        self.cartes.push(*carte);
    }

    pub fn display_last_card(& self) {
        self.cartes[self.cartes.len() - 1].display();
        print!("\n");
    }

    pub fn last_card_played(& self) -> Carte {
        return self.cartes[self.cartes.len() - 1];
    }
}