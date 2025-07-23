# 📚 Rust - Notes d'apprentissage d'un étudiant (7h de cours intensif)

> **Contexte**: Notes prises par un étudiant en systèmes réseaux et cloud computing découvrant Rust pour la première fois. Ces notes vulgarisent les concepts pour une meilleure compréhension.

## 🎯 Objectif du document
Synthétiser les concepts Rust découverts en 7h de cours, avec des exemples pratiques et des parallèles avec le C pour faciliter la compréhension.

---

## 📋 Table des matières
1. [Introduction à Rust](#introduction-à-rust)
2. [Ownership (Propriété)](#ownership-propriété)
3. [Borrowing (Emprunt)](#borrowing-emprunt)
4. [Membership (Appartenance aux structures)](#membership-appartenance)
5. [Gestion des fichiers](#gestion-des-fichiers)
6. [Programmation réseau TCP](#programmation-réseau-tcp)
7. [Concepts avancés](#concepts-avancés)
8. [Comparaisons avec le C](#comparaisons-avec-le-c)

---

## 🦀 Introduction à Rust

### Pourquoi Rust ?
- **Sécurité mémoire** : Pas de segfaults comme en C !
- **Performance** : Aussi rapide que le C
- **Concurrence** : Gestion native des threads
- **Pas de garbage collector** : Contrôle total de la mémoire

### Premier programme
```rust
fn main() {
    println!("Hello, World!");
}
```

**💡 Note d'étudiant** : Contrairement au C, pas besoin d'inclure des headers !

---

## 🔒 Ownership (Propriété)

### Le concept clé de Rust
L'ownership est LE concept qui différencie Rust du C. Imaginez que chaque donnée a un "propriétaire" unique.

### Les 3 règles d'or :
1. **Chaque valeur a un propriétaire unique**
2. **Quand le propriétaire disparaît, la valeur est automatiquement libérée**
3. **Il ne peut y avoir qu'un seul propriétaire à la fois**

### Exemple pratique :
```rust
fn main() {
    let nom = String::from("Amine"); // nom "possède" la String
    let nom2 = nom; // DÉPLACEMENT ! nom n'est plus valide
    
    // println!("{}", nom); // ❌ ERREUR ! nom a été déplacé
    println!("{}", nom2); // ✅ OK
}
```

**🎓 Parallèle avec le C** : 
- En C : `char* nom = malloc(...)` puis `free(nom)` à la fin
- En Rust : Libération automatique quand la variable sort du scope

### Solution : le clonage
```rust
let nom = String::from("Amine");
let nom2 = nom.clone(); // Copie complète
// Maintenant on peut utiliser nom ET nom2
```

---

## 🤝 Borrowing (Emprunt)

### Le concept
Au lieu de déplacer la propriété, on peut "emprunter" temporairement la donnée.

### Emprunts immuables (&)
```rust
fn afficher_nom(nom: &String) { // Emprunt immuable
    println!("Nom: {}", nom);
}

fn main() {
    let nom = String::from("Amine");
    afficher_nom(&nom); // On prête nom à la fonction
    println!("{}", nom); // ✅ nom est toujours utilisable !
}
```

### Emprunts mutables (&mut)
```rust
fn modifier_nom(nom: &mut String) { // Emprunt mutable
    nom.push_str(" (modifié)");
}

fn main() {
    let mut nom = String::from("Amine");
    modifier_nom(&mut nom);
    println!("{}", nom); // "Amine (modifié)"
}
```

**🎓 Parallèle avec le C** :
- Emprunt immuable = `const char*`
- Emprunt mutable = `char*`
- Mais en Rust, c'est vérifié à la compilation !

---

## 🏗️ Membership (Appartenance)

### Structures et leurs données
Le membership décrit quelles données appartiennent à une structure.

```rust
struct User {
    nom: String,      // La struct "possède" ces données
    email: String,    // Elles seront libérées avec la struct
}

impl User {
    fn new(nom: String, email: String) -> User {
        User { nom, email }
    }
    
    // Getter qui emprunte
    fn get_nom(&self) -> &str {
        &self.nom
    }
}
```

**💡 Note d'étudiant** : `&self` = la méthode emprunte l'objet, ne le prend pas !

---

## 📁 Gestion des fichiers

### Structure Fichier complète
Voici notre structure Fichier avec toutes les fonctionnalités demandées :

```rust
use std::fs::File;
use std::io::{self, Write, BufReader, Read};
use chrono::{DateTime, Utc};

pub struct Fichier {
    nom: String,
    contenu: String,
    date_creation: DateTime<Utc>,
}

impl Fichier {
    // Constructeur
    pub fn new(nom: &str, contenu: &str) -> Self {
        Fichier {
            nom: nom.to_string(),
            contenu: contenu.to_string(),
            date_creation: Utc::now(),
        }
    }
    
    // Créer le fichier
    pub fn creer_fichier(&self) -> io::Result<()> {
        let mut file = File::create(&self.nom)?;
        let contenu_avec_date = format!("{}\n\n[Créé le : {}]", 
            self.contenu, 
            self.date_creation.format("%d/%m/%Y %H:%M:%S"));
        file.write_all(contenu_avec_date.as_bytes())?;
        Ok(())
    }
    
    // Lire le fichier
    pub fn lire(&self) -> io::Result<String> {
        let file = File::open(&self.nom)?;
        let mut reader = BufReader::new(file);
        let mut contenu = String::new();
        reader.read_to_string(&mut contenu)?;
        Ok(contenu)
    }
    
    // Getters
    pub fn get_nom(&self) -> &str { &self.nom }
    pub fn get_contenu(&self) -> &str { &self.contenu }
}
```

**🎓 Parallèle avec le C** :
- En C : `FILE* f = fopen(...); fwrite(...); fclose(f)`
- En Rust : Le fichier se ferme automatiquement !

---

## 🌐 Programmation réseau TCP

### Serveur TCP simple
```rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

pub struct ServeurTCP {
    clients: Vec<Client>, // Liste des clients connectés
}

pub struct Client {
    id: u32,
    nom: String,
    adresse: String,
}

impl ServeurTCP {
    pub fn demarrer(&self, adresse: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(adresse)?;
        
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // Gérer chaque client dans un thread
                    std::thread::spawn(move || {
                        Self::gerer_client(stream);
                    });
                }
                Err(e) => println!("Erreur: {}", e),
            }
        }
        Ok(())
    }
}
```

**💡 Note d'étudiant** : En Rust, pas besoin de gérer manuellement les sockets comme en C !

---

## 🚀 Concepts avancés

### Gestion d'erreurs avec Result
```rust
fn operation_qui_peut_echouer() -> Result<String, std::io::Error> {
    let contenu = std::fs::read_to_string("fichier.txt")?;
    Ok(contenu)
}

fn main() {
    match operation_qui_peut_echouer() {
        Ok(contenu) => println!("Succès: {}", contenu),
        Err(erreur) => println!("Erreur: {}", erreur),
    }
}
```

### Gestion des dates avec Chrono
```rust
use chrono::Utc;

fn main() {
    let maintenant = Utc::now();
    println!("Date actuelle: {}", maintenant.format("%d/%m/%Y %H:%M:%S"));
}
```

---

## ⚖️ Comparaisons avec le C

| Aspect | C | Rust |
|--------|---|------|
| **Gestion mémoire** | `malloc()`/`free()` manuel | Automatique avec ownership |
| **Sécurité** | Segfaults possibles | Vérification à la compilation |
| **Concurrence** | Threads avec risques | Threads sécurisés |
| **Gestion erreurs** | Codes de retour | `Result<T, E>` |
| **Fichiers** | `fopen`/`fclose` manuel | Fermeture automatique |

### Exemple de différences :

**En C :**
```c
#include <stdio.h>
#include <stdlib.h>

int main() {
    char* nom = malloc(100);
    strcpy(nom, "Amine");
    printf("%s\n", nom);
    free(nom); // ⚠️ Oubli = fuite mémoire !
    return 0;
}
```

**En Rust :**
```rust
fn main() {
    let nom = String::from("Amine");
    println!("{}", nom);
    // ✅ Libération automatique !
}
```

---

## 🎯 Points clés à retenir

### Les 5 concepts essentiels :
1. **Ownership** : Chaque donnée a un propriétaire unique
2. **Borrowing** : On peut emprunter sans prendre la propriété
3. **Membership** : Les structures possèdent leurs données
4. **Safety** : Rust évite les erreurs de mémoire à la compilation
5. **Result** : Gestion propre des erreurs

### Conseils d'étudiant :
- 🤔 **Au début, c'est déroutant** : Le compilateur Rust semble difficile, mais il nous aide !
- 💡 **Pensez "propriétaire"** : Qui possède cette donnée ? Qui peut la modifier ?
- 🔄 **Utilisez les emprunts** : Évitez les clones inutiles
- 📚 **Lisez les erreurs** : Le compilateur Rust donne d'excellents conseils

---

## 🛠️ Projets réalisés

### 1. Structure Fichier complète
- ✅ Création avec date automatique
- ✅ Lecture/Écriture sécurisée
- ✅ Méthodes de modification
- ✅ Getters pour accès aux données

### 2. Serveur TCP multi-clients
- ✅ Gestion de plusieurs connexions
- ✅ Structure Client avec ownership
- ✅ Threads pour chaque client
- ✅ Liste partagée des clients connectés

---

## 📝 Conclusion

Après 7h de cours intensif, Rust peut sembler complexe venant du C, mais ses concepts garantissent :
- **Zéro segfault** 🛡️
- **Gestion mémoire automatique** 🤖
- **Performance native** ⚡
- **Sécurité à la compilation** 🔒

**Message d'encouragement** : Rust a une courbe d'apprentissage raide, mais une fois maîtrisé, il offre une tranquillité d'esprit incomparable pour développer des systèmes robustes !

---

*📚 Document rédigé par un étudiant en systèmes réseaux découvrant Rust - Juillet 2025*