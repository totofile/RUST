use std::fs::File;
use std::io::{self, Write, BufReader, Read};
use chrono::{DateTime, Utc};


// fn main() -> io::Result<()> {
    
//     let mut  file = File::create("test.txt")?; // créer ou écraser un fichier
//     file.write_all(b"fichier cree ceci est eun exemple")?; // on écrit les données dans le fichier 
//      // b est un byte string on utilise lorsque on travaille avec des données binaires
//     println!("le fichier  ete cree avec succes ");

//     Ok(()) // signifie que tout s'est bien passé et () rien à retourner 
//     //Err(e) // signifie qu'une erreur i/o s'est produite 
// }


pub struct Fichier {
    nom: String,
    contenu: String,
    date_creation: DateTime<Utc>,
}

impl Fichier {
    // Constructeur pour créer une nouvelle instance de Fichier
    pub fn new(nom: &str, contenu: &str) -> Self {
        Fichier {
            nom: nom.to_string(),
            contenu: contenu.to_string(),
            date_creation: Utc::now(),
        }
    }

    // Méthode statique pour créer directement un fichier sans instance
    pub fn creer_avec_nom(nom: &str, contenu: &str) -> io::Result<()> {
        let mut file = File::create(nom)?;
        file.write_all(contenu.as_bytes())?;
        println!("Fichier {} créé directement avec succès", nom);
        Ok(())
    }

    // Méthode pour créer le fichier avec le contenu de la structure
    pub fn creer_fichier(&self) -> io::Result<()> {
        let mut file = File::create(&self.nom)?;
        let contenu_avec_date = format!("{}\n\n[Créé le : {}]", 
            self.contenu, 
            self.date_creation.format("%d/%m/%Y %H:%M:%S"));
        file.write_all(contenu_avec_date.as_bytes())?;
        println!("Le fichier {} a été créé avec succès", self.nom);
        Ok(())
    }

    // Ancienne méthode ecrire (pour compatibilité)
    pub fn ecrire(&self) -> io::Result<()> {
        self.creer_fichier()
    }

    // Méthode pour lire le contenu d'un fichier
    pub fn lire(&self) -> io::Result<String> {
        let file = File::open(&self.nom)?;
        let mut reader = BufReader::new(file);
        let mut contenu = String::new();
        reader.read_to_string(&mut contenu)?;
        Ok(contenu)
    }

    // Modifier le contenu
    pub fn modifier_contenu(&mut self, nouveau_contenu: &str) {
        self.contenu = nouveau_contenu.to_string();
    }

    // Ajouter du contenu
    pub fn ajouter_contenu(&mut self, contenu_additionnel: &str) {
        self.contenu.push_str(contenu_additionnel);
    }

    // Vérifier si le fichier existe
    pub fn existe(&self) -> bool {
        std::fs::metadata(&self.nom).is_ok()
    }

    // Getters
    pub fn get_nom(&self) -> &str {
        &self.nom
    }

    pub fn get_contenu(&self) -> &str {
        &self.contenu
    }

    pub fn get_date_creation(&self) -> &DateTime<Utc> {
        &self.date_creation
    }
}

// A faire:    créer une structure Fichier et implémenter une fonction qui crée un fichier
               // et qui prend en paramètre le nom de fichier 
               // ecrire.rs 
