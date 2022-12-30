pub mod deck;
pub mod defausse;
pub mod joueur;
pub mod carte;
pub mod couleur;
pub mod gestionnaire_de_jeu;
use gestionnaire_de_jeu::GestionnaireDeJeu;

use crate::gestionnaire_de_jeu::build_gestionnaire_jeu;

const NB_CARDS: u8 = 108;

fn main() {
    let mut gestionnaire_jeu: GestionnaireDeJeu = build_gestionnaire_jeu(
        Vec::new(),
        Vec::with_capacity(NB_CARDS.try_into().unwrap()),
        Vec::new()
    );


    println!("Création du Deck");
    gestionnaire_jeu.build_deck();

    gestionnaire_jeu.display_deck();

    gestionnaire_jeu.shuffle_deck();

    gestionnaire_jeu.get_number_of_players();

    gestionnaire_jeu.fill_players();

    gestionnaire_jeu.build_defausse();

    println!("Début de la partie");

    // Tant qu'aucun des joueurs n'a plus de cartes
    loop {
        let mut saute_tour = false;
        // Tant que l'utilisateur n'a pas rentré un choix correct
        loop {
            gestionnaire_jeu.display_last_card_played();

            gestionnaire_jeu.display_player_main();

            let mut choice = gestionnaire_jeu.get_player_choice();

            if choice == 0 {
                gestionnaire_jeu.draw_card();
                break;
            }
            else if choice > 0 && choice < gestionnaire_jeu.nb_card_player() + 1 {
                choice -= 1;
                // On vérifie que la carte est jouable et on récupère son numéro
                let num_card_played = gestionnaire_jeu.check_play(choice);

                if num_card_played == "+4" {
                    gestionnaire_jeu.plus_quatre(choice);
                }
                else if num_card_played == "c" {
                    gestionnaire_jeu.change_couleur(choice);
                }
                else if num_card_played == "+2" {
                    gestionnaire_jeu.plus_deux();
                }
                else if num_card_played == "s" {
                    gestionnaire_jeu.change_sens();
                }
                else if num_card_played == "t" {
                    saute_tour = true;
                }
                else if num_card_played == "" {
                    println!("Carte non jouable!");
                    continue;
                }

                gestionnaire_jeu.remove_card_played(choice);

                // Sort de la boucle
                break;
            }
            else {
                println!("Wrong choice");
            }
        }

        if gestionnaire_jeu.player_has_win() {
            gestionnaire_jeu.display_winner();
            break;
        }
        else {
            if saute_tour {
                gestionnaire_jeu.saute_tour();
            }
            gestionnaire_jeu.next_player();
        }
    }
}
