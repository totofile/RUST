use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

// Structure Client avec ownership et membership
#[derive(Clone, Debug)]
pub struct Client {
    id: u32,
    nom: String,
    adresse: String,
}

impl Client {
    pub fn new(id: u32, nom: String, adresse: String) -> Self {
        Client { id, nom, adresse }
    }

    // Getters
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_nom(&self) -> &str {
        &self.nom
    }

    pub fn get_adresse(&self) -> &str {
        &self.adresse
    }
}

// Structure ServeurTCP
pub struct ServeurTCP {
    clients: Arc<Mutex<HashMap<u32, Client>>>,
    prochain_id: Arc<Mutex<u32>>,
}

impl ServeurTCP {
    pub fn new() -> Self {
        ServeurTCP {
            clients: Arc::new(Mutex::new(HashMap::new())),
            prochain_id: Arc::new(Mutex::new(1)),
        }
    }

    pub fn demarrer(&self, adresse: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(adresse)?;
        println!("Serveur TCP démarré sur {}", adresse);
        println!("En attente de connexions...");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let clients = Arc::clone(&self.clients);
                    let prochain_id = Arc::clone(&self.prochain_id);
                    
                    // Chaque client est géré dans un thread séparé
                    thread::spawn(move || {
                        Self::gerer_client(stream, clients, prochain_id);
                    });
                }
                Err(e) => {
                    println!("Erreur de connexion: {}", e);
                }
            }
        }
        Ok(())
    }

    fn gerer_client(
        mut stream: TcpStream,
        clients: Arc<Mutex<HashMap<u32, Client>>>,
        prochain_id: Arc<Mutex<u32>>,
    ) {
        let adresse_client = stream.peer_addr().unwrap().to_string();
        
        // Générer un ID unique pour le client
        let id = {
            let mut id_counter = prochain_id.lock().unwrap();
            let current_id = *id_counter;
            *id_counter += 1;
            current_id
        };

        println!("Nouveau client connecté: {} (ID: {})", adresse_client, id);

        // Envoyer un message de bienvenue
        let message_bienvenue = format!("Bienvenue sur le serveur TCP ! Votre ID est {}\n", id);
        let _ = stream.write_all(message_bienvenue.as_bytes());

        // Demander le nom du client
        let _ = stream.write_all(b"Veuillez entrer votre nom: ");
        
        let mut buffer = [0; 1024];
        let mut nom_client = String::new();
        
        match stream.read(&mut buffer) {
            Ok(taille) => {
                nom_client = String::from_utf8_lossy(&buffer[..taille]).trim().to_string();
                if nom_client.is_empty() {
                    nom_client = format!("Client_{}", id);
                }
            }
            Err(_) => {
                nom_client = format!("Client_{}", id);
            }
        }

        // Créer le client et l'ajouter à la liste
        let client = Client::new(id, nom_client.clone(), adresse_client.clone());
        {
            let mut clients_lock = clients.lock().unwrap();
            clients_lock.insert(id, client.clone());
            println!("Client ajouté: {} - {} (ID: {})", client.get_nom(), client.get_adresse(), client.get_id());
        }

        // Envoyer la confirmation
        let confirmation = format!("Salut {} ! Tu es maintenant connecté.\n", nom_client);
        let _ = stream.write_all(confirmation.as_bytes());

        // Afficher la liste des clients connectés
        Self::afficher_clients_connectes(&clients);

        // Garder la connexion ouverte et gérer les messages
        loop {
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(0) => {
                    // Client déconnecté
                    println!("Client {} (ID: {}) s'est déconnecté", nom_client, id);
                    break;
                }
                Ok(taille) => {
                    let message = String::from_utf8_lossy(&buffer[..taille]);
                    println!("Message de {} (ID: {}): {}", nom_client, id, message.trim());
                    
                    // Répondre au client
                    let reponse = format!("Message reçu: {}", message.trim());
                    let _ = stream.write_all(reponse.as_bytes());
                }
                Err(_) => {
                    println!("Erreur de lecture pour le client {} (ID: {})", nom_client, id);
                    break;
                }
            }
        }

        // Supprimer le client de la liste lors de la déconnexion
        {
            let mut clients_lock = clients.lock().unwrap();
            clients_lock.remove(&id);
            println!("Client {} (ID: {}) supprimé de la liste", nom_client, id);
        }

        Self::afficher_clients_connectes(&clients);
    }

    fn afficher_clients_connectes(clients: &Arc<Mutex<HashMap<u32, Client>>>) {
        let clients_lock = clients.lock().unwrap();
        println!("\n=== Clients connectés ({}) ===", clients_lock.len());
        for (_, client) in clients_lock.iter() {
            println!("- {} (ID: {}) depuis {}", client.get_nom(), client.get_id(), client.get_adresse());
        }
        println!("========================\n");
    }
}

// Fonction principale pour tester le serveur
pub fn main() -> std::io::Result<()> {
    let serveur = ServeurTCP::new();
    serveur.demarrer("127.0.0.1:8080")
}