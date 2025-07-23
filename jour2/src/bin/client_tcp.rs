use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    println!("Connexion au serveur TCP...");
    
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            println!("Connecté au serveur !");
            
            // Lire le message de bienvenue
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(taille) => {
                    let message = String::from_utf8_lossy(&buffer[..taille]);
                    print!("{}", message);
                }
                Err(e) => println!("Erreur de lecture: {}", e),
            }

            // Répondre avec notre nom
            let nom = "Etudiant_Test";
            stream.write_all(nom.as_bytes())?;

            // Lire la confirmation
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(taille) => {
                    let message = String::from_utf8_lossy(&buffer[..taille]);
                    print!("{}", message);
                }
                Err(e) => println!("Erreur de lecture: {}", e),
            }

            // Envoyer quelques messages de test
            let messages = vec![
                "Bonjour serveur !",
                "Comment ça va ?",
                "Test de message",
                "Au revoir !"
            ];

            for msg in messages {
                thread::sleep(Duration::from_secs(2));
                stream.write_all(msg.as_bytes())?;
                
                // Lire la réponse
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(taille) => {
                        let reponse = String::from_utf8_lossy(&buffer[..taille]);
                        println!("Serveur: {}", reponse);
                    }
                    Err(e) => println!("Erreur de lecture: {}", e),
                }
            }

            println!("Déconnexion...");
        }
        Err(e) => {
            println!("Impossible de se connecter au serveur: {}", e);
            println!("Assurez-vous que le serveur est démarré !");
        }
    }
    
    Ok(())
}