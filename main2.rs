use std::io::{self, Write};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Stock {
    quantite: i32,
}

struct Commande {
    id: usize,
    duree_unites: u64,
    quantite_a_retirer: i32,
}

fn main() {
    const NOMBRE_THREADS: usize = 5; //modifiable

    let stock = Arc::new(Mutex::new(Stock { quantite: 100 })); //modifiable
    let (tx, rx) = mpsc::channel::<Commande>();
    let rx_partage = Arc::new(Mutex::new(rx));

    // --- CRÉATION DES 5 SERVEURS EN ARRIÈRE-PLAN ---
    for id_thread in 1..=NOMBRE_THREADS {
        let rx_clone = Arc::clone(&rx_partage);
        let stock_clone = Arc::clone(&stock);

        thread::spawn(move || {
            loop {
                let commande = {
                    let rx_verrou = rx_clone.lock().unwrap();
                    match rx_verrou.recv() {
                        Ok(cmd) => cmd,
                        Err(_) => break, // Arrêt si tx est détruit
                    }
                };

                println!(
                    "\n([Serveur {}] Traitement de la commande n°{} (durée: {}s)...)",
                    id_thread, commande.id, commande.duree_unites
                );

                thread::sleep(Duration::from_secs(commande.duree_unites));

                {
                    let mut stock_verrou = stock_clone.lock().unwrap();
                    if stock_verrou.quantite >= commande.quantite_a_retirer {
                        stock_verrou.quantite -= commande.quantite_a_retirer;
                        println!(
                            "\n([Serveur {}] Commande n°{} OK. Stock restant : {})",
                            id_thread, commande.id, stock_verrou.quantite
                        );
                    } else {
                        println!(
                            "\n([Serveur {}] ÉCHEC commande n°{} : Stock insuffisant ({}))",
                            id_thread, commande.id, stock_verrou.quantite
                        );
                    }
                }
            }
        });
    }

    // --- LE THREAD PRINCIPAL DEVIENT L'ÉMETTEUR DU TERMINAL ---
    println!("=== TERMINAL SERVEUR DE COMMANDES ===");
    println!("Saisissez les commandes sous le format : <DURÉE> <QUANTITÉ>");
    println!("Exemple : '3 10' (Durée 3s, Quantité 10)");
    println!("Tapez 'quitter' pour fermer le programme.\n");

    let mut compteur_id = 1;

    loop {
        print!("Entrez une commande (id {}) ou tapez quitter > ", compteur_id);
        // Force l'affichage immédiat du prompt "Entrez une commande > "
        io::stdout().flush().unwrap();

        let mut saisie = String::new();
        io::stdin().read_line(&mut saisie).unwrap();
        let texte = saisie.trim();

        if texte == "quitter" {
            println!("Fermeture du programme...");
            break;
        }

        // Découpage de la ligne saisie ("3 10" -> durée: 3, quantité: 10)
        let parties: Vec<&str> = texte.split_whitespace().collect(); //collect transforme l'itérateur en tableau
        if parties.len() == 2 {
            if let (Ok(duree), Ok(quantite)) = (parties[0].parse::<u64>(), parties[1].parse::<i32>()) { //parse tranforme les "int" en Option(int)
                let cmd = Commande {
                    id: compteur_id,
                    duree_unites: duree,
                    quantite_a_retirer: quantite,
                };

                // ENVOI DANS LE CANAL TX
                tx.send(cmd).unwrap();
                println!("--> Commande n°{} envoyée au canal !", compteur_id);
                compteur_id += 1;
            } else {
                println!(" Erreur : Veuillez entrer des nombres entiers valides.");
            }
        } else {
            println!(" Format incorrect. Exemple de saisie valide : 2 15");
        }
    }
}