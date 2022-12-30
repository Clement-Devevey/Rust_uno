pub mod deck;
pub mod defausse;
pub mod joueur;
pub mod carte;
pub mod couleur;
pub mod gestionnaire_de_jeu;
use gestionnaire_de_jeu::GestionnaireDeJeu;

use crate::gestionnaire_de_jeu::build_gestionnaire_jeu;

const NB_OF_CARDS: i8 = 40;

fn main() {
    let mut gestionnaire_jeu: GestionnaireDeJeu = build_gestionnaire_jeu(
        Vec::new(),
        Vec::with_capacity(NB_OF_CARDS.try_into().unwrap()),
        Vec::new()
    );


    println!("Création du Deck");
    gestionnaire_jeu.build_deck();

    gestionnaire_jeu.shuffle_deck();

    gestionnaire_jeu.get_number_of_players();

    gestionnaire_jeu.fill_players();

    gestionnaire_jeu.build_defausse();

    println!("Début de la partie");

    loop {

        loop {
            gestionnaire_jeu.display_last_card_played();

            gestionnaire_jeu.display_player_main();

            let choice = gestionnaire_jeu.get_player_choice();

            if choice == 0 {
                gestionnaire_jeu.draw_card();
                break;
            }
            else if choice > 0 && choice < gestionnaire_jeu.nb_card_player() + 1 {
                if gestionnaire_jeu.check_play(choice - 1) {
                    break;
                }
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
            gestionnaire_jeu.next_player();
        }
    }



    // println!("Quelle carte souhaites tu jouer ?");
    // let mut choix: usize;
    // scan!("{}", choix);
    // choix-=1;
    
    // println!("Carte sélectionnée:");
    // joueurs[0].main[choix].display();
    // print!("\n");

    // if joueurs[0].choice_is_playable(&choix, &mut defausse.last_card_played()) {
    //     println!("Choix OK");
    //     defausse.ajouter_carte(&joueurs[0].main.remove(choix));
    // }

    // println!("Dernière carte jouée:");
    // defausse.display_last_card();

    // for joueur in joueurs.iter() {
    //     joueur.display_main();
    // }

}
