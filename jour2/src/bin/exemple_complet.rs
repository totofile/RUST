// Exemple complet combinant la structure Fichier et les concepts d'ownership/membership

use std::io;
use jour2tp::ecrire::Fichier;
use chrono::Utc;

// Structure User pour démonstrer l'ownership
struct User {
    nom: String,
    secu: String,
}

impl User {
    fn new(nom: String, secu: String) -> Self {
        User { nom, secu }
    }
}

fn main() -> io::Result<()> {
    println!("=== Démonstration des concepts Rust apprises ===\n");

    // 1. Test de la structure Fichier complète
    println!("1. Test de la structure Fichier:");
    test_fichier()?;

    // 2. Démonstration de l'ownership
    println!("\n2. Démonstration de l'ownership:");
    demo_ownership();

    // 3. Test des emprunts
    println!("\n3. Test des emprunts:");
    demo_emprunts();

    // 4. Membership avec structures
    println!("\n4. Membership avec structures:");
    demo_membership();

    println!("\n=== Fin des démonstrations ===");
    Ok(())
}

fn test_fichier() -> io::Result<()> {
    // Créer un fichier avec date
    let mut fichier = Fichier::new(
        "demo.txt",
        "Fichier de démonstration créé par l'étudiant"
    );

    println!("  - Fichier créé: {}", fichier.get_nom());
    println!("  - Date: {}", fichier.get_date_creation().format("%d/%m/%Y %H:%M:%S"));
    
    // Tester les méthodes
    fichier.ajouter_contenu("\n  - Contenu ajouté par ajouter_contenu()");
    fichier.creer_fichier()?;
    
    if fichier.existe() {
        println!("  - Le fichier existe !");
        let contenu = fichier.lire()?;
        println!("  - Contenu:\n{}", contenu);
    }

    // Test de la méthode statique
    Fichier::creer_avec_nom("test_statique.txt", "Fichier créé avec méthode statique")?;
    
    Ok(())
}

fn demo_ownership() {
    // 1. Ownership de base
    let prenom = String::from("Amine");
    let secu = String::from("198977787");
    
    // Avec clone pour éviter le déplacement
    let prenom2 = prenom.clone();
    greetings(prenom); // ownership déplacé
    // println!("{}", prenom); // Erreur ! ownership déplacé
    println!("  - Clone de prenom: {}", prenom2);
    
    // 2. Avec emprunt &
    greetings2(&secu); // emprunt immuable
    println!("  - Secu après emprunt: {}", secu); // OK !
}

fn demo_emprunts() {
    let mut texte = String::from("Hello");
    
    // Emprunt immuable
    let longueur = calculer_longueur(&texte);
    println!("  - Longueur de '{}': {}", texte, longueur);
    
    // Emprunt mutable
    modifier_texte(&mut texte);
    println!("  - Texte modifié: {}", texte);
}

fn demo_membership() {
    // Création d'un User avec membership
    let user = User::new(
        String::from("Amine"),
        String::from("1879716171009 55")
    );
    
    // La fonction display prend ownership et le retourne
    let user_retourne = display(user);
    
    // On peut encore utiliser user_retourne car il a été retourné
    println!("  - User récupéré: {} - {}", user_retourne.nom, user_retourne.secu);
}

// Fonctions utilitaires
fn greetings(msg: String) {
    println!("  - Hello mister, {}", msg);
}

fn greetings2(msg: &String) {
    println!("  - Hello mister (emprunt), {}", msg);
}

fn calculer_longueur(s: &String) -> usize {
    s.len()
}

fn modifier_texte(s: &mut String) {
    s.push_str(" World!");
}

fn display(u: User) -> User {
    println!("  - Nom: {}, Secu: {}", u.nom, u.secu);
    u // Retourne l'ownership
}