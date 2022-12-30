use crate::couleur::Couleur;
use crate::deck::Deck;
use crate::carte::Carte;
use crate::joueur::{Joueur, build_main};
use crate::defausse::Defausse;
use colored::Colorize;

use text_io::scan;
pub struct GestionnaireDeJeu {
    joueurs: Vec<Joueur>,
    deck: Deck,
    defausse: Defausse,
    nb_of_players: u8,
    current_player: u8,
    sens: bool,
}

pub fn build_gestionnaire_jeu(joueurs: Vec<Joueur>, cartes_deck: Vec<Carte>, cartes_defausse: Vec<Carte>) -> GestionnaireDeJeu {
    GestionnaireDeJeu { 
        joueurs: joueurs, 
        deck: Deck{cartes: cartes_deck}, 
        defausse: Defausse {cartes: cartes_defausse},
        nb_of_players: 0,
        current_player: 0,
        sens: true,
    }
}

impl GestionnaireDeJeu {
    pub fn build_deck(&mut self) {
        self.deck.build_deck();
    }

    pub fn display_deck(& self) {
        self.deck.display();
        println!("Len: {}", self.deck.cartes.len());
    }

    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle();
    }

    pub fn get_number_of_players(&mut self) {
        let mut str_nb_players: String;
        loop {
            print!("Number of player:\n");
            scan!("{}", str_nb_players);
            
            match str_nb_players.parse::<u8>() {
                Ok(nb_of_players) => {
                    if nb_of_players > 1 {
                        self.nb_of_players = nb_of_players;
                        println!("Number of players sets to {}", self.nb_of_players);
                        break;
                    } else {
                        println!("Invalid entry");
                    }
                },
                Err(_) => println!("Invalid entry"),
            }
        }
    }

    //ask for names and fill their cards
    pub fn fill_players(&mut self) {
        let mut name: String;

        for i in 0..self.nb_of_players as usize {
            println!("Player {} what is your name:", i+1);
            scan!("{}", name);
            println!("Your name is set to {name}");
            println!("--------------------------");
            self.joueurs.push(
                Joueur{main: build_main(&mut self.deck), 
                nom: name, 
                score: 0}
            );
        }
    }

    pub fn display_mains(& self) {
        for joueur in self.joueurs.iter() {
            joueur.display_main();
        }
    }

    pub fn build_defausse(&mut self) {
        let mut first_card: Carte;
        // On check que l'on ne tombe pas sur une carte spécial pour le tour 1 sinon on la remet au fond du paquet
        loop {
            first_card = self.deck.draw_card();
            if first_card.couleur != Couleur::Noir {
                break;
            }
            self.defausse.cartes.insert(0, first_card);
        }
        self.defausse.ajouter_carte(&first_card);
    }

    pub fn display_last_card_played(& self) {
        println!("Dernière carte jouée:");
        self.defausse.display_last_card();
        println!("--------------------------");
    }

    pub fn is_there_a_winner(& self) -> bool {
        for joueur in self.joueurs.iter() {
            if joueur.get_nb_card() == 0 {
                return true;
            }
        }
        return false;
    }

    pub fn get_player_choice(& self) -> u8 {
        loop {
            println!("Quelle carte souhaites tu jouer ? (0 pour tirer une carte)");
            let choix: String;
            scan!("{}", choix);

            match choix.parse::<u8>() {
                Ok(choice) => {
                    return choice;
                },
                Err(_) => println!("Invalid entry"),
            }
        }
    }

    pub fn get_last_card_played(&mut self) -> Carte {
        self.defausse.last_card_played()
    }

    pub fn check_play(&mut self, choix: u8) -> String {
        let last_card: Carte = self.get_last_card_played();
        
        if self.current_player().choice_is_playable(choix, &last_card) {
            println!("{}", "Choix OK".green());
            return self.current_player().main[choix as usize].numero.clone();
        }

        println!("{}", "Choix non OK".red());
        return "".to_string();
    }

    pub fn remove_card_played(&mut self, index: u8) {
        let carte_to_remove = self.current_player().main.remove(index as usize);
        self.defausse.ajouter_carte(&carte_to_remove);
    }

    pub fn display_card_selected(& self, choice: u8) {
        println!("Carte sélectionnée:");
        self.joueurs[self.current_player as usize].main[choice as usize].display();
        println!("\n--------------------------");
    }

    pub fn next_player(&mut self) {
        if self.sens {
            self.current_player += 1;
            if self.current_player >= self.nb_of_players {
                self.current_player = 0;
            }
        }
        else {
            if self.current_player <= 0 {
                self.current_player = self.nb_of_players;
            }
            self.current_player -= 1;
        }
    }

    pub fn display_player_main(& mut self) {
        self.current_player().display_main();
    }

    pub fn current_player(&mut self) -> &mut Joueur {
        &mut self.joueurs[self.current_player as usize]
    }

    pub fn nb_card_player(&mut self) -> u8 {
        self.current_player().main.len() as u8
    }

    pub fn draw_card(&mut self) {
        let card_draw = self.deck.draw_card();
        self.current_player().main.push(card_draw);
    }

    pub fn player_has_win(&mut self) -> bool {
        self.current_player().main.len() == 0
    }

    pub fn display_winner(&mut self) {
        println!("Congratulation, {} you won.", self.current_player().nom);
    }

    pub fn plus_deux(&mut self) {
        let next_player: u8 = self.get_next_player();
        self.add_cards_to_player(next_player, 2);
    }

    pub fn get_next_player(&self) -> u8 {
        let next_player: u8;
        if self.sens {
            if {self.current_player + 1} >= self.nb_of_players {
                next_player = 0;
            }
            else {
                next_player = self.current_player + 1;
            }
        }
        else {
            if self.current_player <= 0 {
                next_player = self.nb_of_players - 1;
            } 
            else {
                next_player = self.current_player - 1;
            }
        }
        return next_player;
    }

    pub fn add_cards_to_player(&mut self, next_player: u8, nb_cards: u8) {
        for _i in 0..nb_cards {
            self.joueurs[next_player as usize].main.push(self.deck.draw_card());
        }
    }

    pub fn plus_quatre(&mut self, choice: u8) {
        let next_player: u8 = self.get_next_player();
        self.add_cards_to_player(next_player, 4);
        self.change_couleur(choice);
    }

    pub fn change_couleur(&mut self, choice: u8) {
        let mut user_new_color: String;

        loop {
            println!("Changement de couleur. Choississez votre vouleur [b,j,r,v]:");
            scan!("{}", user_new_color);

            match user_new_color.as_str() {
                "b" => {
                    self.current_player().main[choice as usize].couleur = Couleur::Bleu;
                    break;
                },
                "j" => {
                    self.current_player().main[choice as usize].couleur = Couleur::Jaune;
                    break;
                },
                "r" => {
                    self.current_player().main[choice as usize].couleur = Couleur::Rouge;
                    break;
                },
                "v" => {
                    self.current_player().main[choice as usize].couleur = Couleur::Vert;
                    break;
                },
                _ => {println!("Wrong color choice.")}
            }
        }
    }

    pub fn change_sens(&mut self) {
        self.sens = !self.sens;
    }

    pub fn saute_tour(&mut self) {
        self.next_player();
    }

}