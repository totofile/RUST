// Un petit programme pour gérer des comptes bancaires
use std::io;

// Un trait pour afficher les infos d'un compte (c'est comme une interface)
trait Afficher {
    fn afficher(&self);
}

// La structure d'un compte bancaire
struct CompteBancaire {
    nom: String,
    solde: f64,
    numero_compte: u32,
}

impl CompteBancaire {
    // Créer un nouveau compte
    fn nouveau_compte(nom: String, solde_initial: f64, numero: u32) -> CompteBancaire {
        CompteBancaire {
            nom,
            solde: solde_initial,
            numero_compte: numero,
        }
    }

    // Déposer de l'argent
    fn deposer_des_sous(&mut self, montant: f64) {
        self.solde += montant;
        println!("Depot de {} euros sur le compte de {}", montant, self.nom);
    }

    // Retirer de l'argent
    fn retirer_des_sous(&mut self, montant: f64) {
        if self.solde >= montant {
            self.solde -= montant;
            println!("Retrait de {} euros pour {}", montant, self.nom);
        } else {
            println!("Pas assez de fric ! Solde : {} euros", self.solde);
        }
    }

    // Fermer le compte
    fn fermer_le_compte(&mut self) {
        println!("Fermeture du compte de {} (Solde : {} euros)", self.nom, self.solde);
    }
}

// On utilise le trait pour afficher un compte
impl Afficher for CompteBancaire {
    fn afficher(&self) {
        println!("Compte n°{} - {} - Solde : {} euros", 
                 self.numero_compte, self.nom, self.solde);
    }
}

fn main() {
    println!("=== GESTION COMPTES BANCAIRES ===");
    
    let mut mes_comptes: Vec<CompteBancaire> = Vec::new();
    
    // On crée 3 comptes par défaut
    mes_comptes.push(CompteBancaire::nouveau_compte(
        String::from("Jean-Michel"), 2500.0, 1001
    ));
    mes_comptes.push(CompteBancaire::nouveau_compte(
        String::from("Marie-Claire"), 1200.0, 1002  
    ));
    mes_comptes.push(CompteBancaire::nouveau_compte(
        String::from("Bob"), 42.0, 1003
    ));
    
    let mut numero_suivant = 1004;
    
    loop {
        println!("\n--- MENU ---");
        println!("1. Voir tous les comptes");
        println!("2. Deposer argent");
        println!("3. Retirer argent");
        println!("4. Creer nouveau compte");
        println!("5. Fermer un compte");
        println!("6. Quitter");
        print!("Votre choix : ");

        // On lit le choix de l'utilisateur
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur lecture");

        let choix_num: u32 = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Saisissez un nombre !");
                continue;
            }
        };

        match choix_num {
            1 => {
                println!("\n=== LISTE DES COMPTES ===");
                for compte in &mes_comptes {
                    compte.afficher();
                }
            },
            
            2 => {
                if mes_comptes.is_empty() {
                    println!("Aucun compte !");
                    continue;
                }
                
                println!("Choisir compte :");
                for (i, compte) in mes_comptes.iter().enumerate() {
                    println!("{}. {}", i + 1, compte.nom);
                }
                
                let mut choix_compte = String::new();
                io::stdin().read_line(&mut choix_compte).expect("Erreur");
                
                let index: usize = match choix_compte.trim().parse::<usize>() {
                    Ok(num) if num > 0 && num <= mes_comptes.len() => num - 1,
                    _ => {
                        println!("Numero invalide !");
                        continue;
                    }
                };
                
                println!("Montant a deposer :");
                let mut montant_str = String::new();
                io::stdin().read_line(&mut montant_str).expect("Erreur");
                
                let montant: f64 = match montant_str.trim().parse() {
                    Ok(num) if num > 0.0 => num,
                    _ => {
                        println!("Montant invalide !");
                        continue;
                    }
                };
                
                mes_comptes[index].deposer_des_sous(montant);
            },
            
            3 => {
                if mes_comptes.is_empty() {
                    println!("Aucun compte !");
                    continue;
                }
                
                println!("Choisir compte :");
                for (i, compte) in mes_comptes.iter().enumerate() {
                    println!("{}. {} ({}€)", i + 1, compte.nom, compte.solde);
                }
                
                let mut choix_compte = String::new();
                io::stdin().read_line(&mut choix_compte).expect("Erreur");
                
                let index: usize = match choix_compte.trim().parse::<usize>() {
                    Ok(num) if num > 0 && num <= mes_comptes.len() => num - 1,
                    _ => {
                        println!("Numero invalide !");
                        continue;
                    }
                };
                
                println!("Montant a retirer :");
                let mut montant_str = String::new();
                io::stdin().read_line(&mut montant_str).expect("Erreur");
                
                let montant: f64 = match montant_str.trim().parse() {
                    Ok(num) if num > 0.0 => num,
                    _ => {
                        println!("Montant invalide !");
                        continue;
                    }
                };
                
                mes_comptes[index].retirer_des_sous(montant);
            },
            
            4 => {
                println!("Nom du titulaire :");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).expect("Erreur");
                let nom = nom.trim().to_string();
                
                println!("Solde initial :");
                let mut solde_str = String::new();
                io::stdin().read_line(&mut solde_str).expect("Erreur");
                
                let solde: f64 = match solde_str.trim().parse() {
                    Ok(num) if num >= 0.0 => num,
                    _ => {
                        println!("Solde invalide !");
                        continue;
                    }
                };
                
                let nouveau_compte = CompteBancaire::nouveau_compte(nom, solde, numero_suivant);
                println!("Compte cree avec numero : {}", numero_suivant);
                mes_comptes.push(nouveau_compte);
                numero_suivant += 1;
            },
            
            5 => {
                if mes_comptes.is_empty() {
                    println!("Aucun compte a fermer !");
                    continue;
                }
                
                println!("Compte a fermer :");
                for (i, compte) in mes_comptes.iter().enumerate() {
                    println!("{}. {}", i + 1, compte.nom);
                }
                
                let mut choix_compte = String::new();
                io::stdin().read_line(&mut choix_compte).expect("Erreur");
                
                let index: usize = match choix_compte.trim().parse::<usize>() {
                    Ok(num) if num > 0 && num <= mes_comptes.len() => num - 1,
                    _ => {
                        println!("Numero invalide !");
                        continue;
                    }
                };
                
                mes_comptes[index].fermer_le_compte();
                mes_comptes.remove(index);
                println!("Compte ferme !");
            },
            
            6 => {
                println!("Au revoir !");
                break;
            },
            
            _ => {
                println!("Choix invalide !");
            }
        }
    }
}