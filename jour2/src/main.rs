// A faire:    créer une structure Fichier et implémenter une fonction qui crée un fichier
               // et qui prend en paramètre le nom de fichier 
               // ecrire.rs 

use std::io;
use jour2tp::ecrire::Fichier;

fn main() -> io::Result<()> {
    
    // Créer une instance de Fichier avec les nouvelles signatures
    let mut fichier = Fichier::new(
        "test.txt",
        "Fichier cree avec la structure Fichier - ceci est un exemple"
    );
    
    // Créer le fichier
    fichier.ecrire()?;
    
    // Tester les nouvelles fonctionnalités
    println!("Nom du fichier: {}", fichier.get_nom());
    println!("Date de création: {}", fichier.get_date_creation().format("%d/%m/%Y %H:%M:%S"));
    
    // Ajouter du contenu
    fichier.ajouter_contenu("\n\nContenu ajouté par la méthode ajouter_contenu()");
    fichier.creer_fichier()?; // Recréer le fichier avec le contenu modifié
    
    // Vérifier si le fichier existe
    if fichier.existe() {
        println!("Le fichier existe bien !");
        
        // Lire le contenu du fichier
        match fichier.lire() {
            Ok(contenu) => println!("Contenu lu:\n{}", contenu),
            Err(e) => println!("Erreur lors de la lecture: {}", e),
        }
    }
    Ok(())
}

// Dépendances externes  que vous pouvez utiliser dans  cargo.toml 
// chrono → Gestion des dates et timestamps
// serde → Sérialisation des structures Rust
// serde_json → Format JSON spécifiquement
// tempfile → Fichiers temporaires pour les tests

// [package]
// name = "jour2tp"
// version = "0.1.0"
// edition = "2024"

// [dependencies]
// chrono = {version ="0.4", features=["serde" , "clock"]}
// serde = {version ="1.0", features=["derive"]}
// serde_json = "1.0"

// Exemple : 

// use chrono::Utc; 

// fn main(){
    
//    let  now  = Utc::now();
//    println!("heure actuelle UTC est {}",now); 

//    println!("Format fr : {}",now.format("%d/%m/%Y")); // Format fr : 23/07/2025
//    println!("Format fr : {}",now.format("%d/%m/%Y %H:%M:%S")); // Format fr : 23/07/2025 10:04:32
// }





// // Implémenter la fonction lire dans votre struct Fichier  Tp précédent 

// // lecture à partir d'un fichier
// // dans notre cas on utilise Read et BufReader

// use std::fs::File; // stream 
// use std::io::{self,BufReader,Read};

// fn main() -> io::Result<()>{
     
//        let file = File::open("test.txt")?;
//        let mut reader = BufReader::new(file); // on crée un lecteur tamponné 
//        let mut content = String::new(); 
//        reader.read_to_string(&mut content)?;
       
//        println!("ceci est le contenu du fichier {}", content);

//       // pour mettre en pause le terminal  lire.exe
//       let mut pause = String::new();
//       let _= io::stdin().read_line(&mut pause);


//     Ok(())
// }



// use chrono::Utc; 

//            struct User{
//             nom:String,
//             secu: String
//            }


// fn main(){
    
//    let  now  = Utc::now();
//    println!("heure actuelle UTC est {}",now); 

//    println!("Format fr : {}",now.format("%d/%m/%Y")); // Format fr : 23/07/2025
//    println!("Format fr : {}",now.format("%d/%m/%Y %H:%M:%S")); // Format fr : 23/07/2025 10:04:32

// //  Notions d'ownership ( propriété)  et membership ( appartenance à une structure ) => pour garantir 
// // la sécurité mémoire 

// // 1.Ownership : 
// // chaque valeur a un propriétaire unique, responsable de libérer la mémoire lorsqu'elle sort du scop
//  // quand le propriétaire est déplacé, l'ancien propriétaire ne peut plus y accéder
//  // quand le propriétaire sort su scoê, la valeur est automatiquement libérée  

//  // exemple
//    let prenom = String::from("Amine"); // prenom est proprietaire de la String
//     let secu = String::from("198977787"); // prenom est proprietaire de la String
//    // avec clone
//    let prenom2 = prenom.clone();
//    greetings(prenom); // propriétaire est transféré à la fontion greetings()
//    // println!("{}",prenom); //   non Erreur : ownership déplacé !!!!

//    println!(" clone de prenom : {}",prenom2);


//     // 2 avec emprunt & 
//       greetings2(&secu); //  emprunt immuable 
//       println!("{}", secu); // oui pas d'Erreur 


//            // 3  Membership ( Appartenance à une structure )
//            //   décrit quelles sont les données contenues dans une structure Struct 

//            // exemple : 


//            let user = User{
//             nom: String::from("Amine"),
//             secu: String::from("1879716171009 55"),

//            }; 

//               display(user); 


// }

// fn display(u : User) -> User {

//     println!("Nom {},  secu : {}",u.nom, u.secu); 
//      u 
// }



// fn greetings(msg:String){
//     println!("Hello  mister, {}",msg);
// }

// // avec emprunt & 
// fn greetings2(msg:&String){
//     println!("Hello  mister, {}",msg);
// }

// Tp : Reséau TCP avec gestion clients
// >> créer un serveur TCP  simple qui accepte plusieurs connexions et garde une liste des clients connectés ( une structure Client )
//    vous appliquer  Ownership et Membership 
//    consignes  utiliser la bibliothèque  use std::net::TcpStream  et TcpListner  
//                                         et std::io:: {Read, Write}

// Bonus complément 
// Une structure Fichier  avec plusieurs fonctionnalités :
// Caractéristiques principales :

// -Structure Fichier : Contient le nom du fichier et son contenu
// -Constructeur new() : Pour créer une nouvelle instance
// -Méthode creer_fichier() : Crée le fichier avec le contenu de la structure
// -Méthode statique creer_avec_nom() : Crée directement un fichier sans instance
// - rajouter une date de création 

// Méthodes :
// modifier_contenu() : Changer le contenu
// ajouter_contenu() : Ajouter du contenu
// existe() : Vérifier si le fichier existe
// Getters pour accéder aux propriétés

// Consignes :
// exemple de getter

//    // Getter pour le nom
//     pub fn get_nom(&self) -> &str {
//         &self.nom
//     }


// exemple de constructeur avec new :
//  // Constructeur pour créer une nouvelle instance de Fichier
//     pub fn new(nom: &str, contenu: &str) -> Self {
//         Fichier {
//             nom: nom.to_string(),
//             contenu: contenu.to_string(),
//         }
//     }



