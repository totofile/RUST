# Mes Premières Découvertes en Rust
*Synthèse de 7h de cours intensif - Par un étudiant en systèmes/réseaux qui découvre la programmation système*

## Introduction - Pourquoi Rust ?

Venant d'un background systèmes/réseaux, Rust c'est le langage qui veut résoudre les problèmes du C (fuites mémoire, plantages) tout en gardant les performances. En gros : "La vitesse du C, la sécurité de haut niveau".

Le prof nous fait souvent des parallèles avec le C pour qu'on comprenne mieux, et franchement ça aide.

---

## 1. Les Variables - Les Bases Qui Changent Tout

### L'Immutabilité par Défaut - Un Concept Révolutionnaire

```rust
let nom = "Erwan";           // Impossible de modifier
let mut age = 23;            // Modifiable avec 'mut'
```

**Pourquoi c'est révolutionnaire ?** 
En C, on peut modifier n'importe quoi n'importe quand, ce qui cause des bugs. Rust nous force à réfléchir : "Est-ce que j'ai vraiment besoin de modifier cette variable ?"

### Types de Données - Plus Précis qu'en C

```rust
let age: u32 = 23;           // Entier non-signé (0 à 4 milliards)
let temperature: f32 = 21.5; // Flottant 32 bits
let compteur: i32 = -10;     // Entier signé (-2 milliards à +2 milliards)
```

**Convention importante :** On utilise le snake_case (age_chien) jamais de CamelCase comme en Java.

**Types courants :**
- `i32` : entier signé par défaut
- `u32` : entier non signé 
- `f32/f64` : nombres flottants
- `u8` : de 0 à 255 (pratique pour les octets en réseau !)

---

## 2. Les Fonctions - Simple et Efficace

```rust
fn addition(a: i32, b: i32) -> i32 {
    a + b  // PAS de point-virgule = valeur de retour
}

fn say_hello(nom: &str) {  // &str = référence vers une chaîne
    println!("Bonjour, {}", nom);
}
```

**Grande différence avec C :**
- En C : `return a + b;`
- En Rust : `a + b` (sans return ni point-virgule)

Le `&str` c'est une référence vers une chaîne, on verra ça plus en détail avec les structures.

---

## 3. Conditions et Boucles - Du Familier

### Conditions Classiques
```rust
let nombre = 16;
if nombre % 2 == 0 {
    println!("{} est pair", nombre);
} else {
    println!("nombre impair");
}
```

### Boucles For avec des Intervalles Pratiques

```rust
for i in 1..=10 {        // Inclusif : 1 à 10
    println!("i vaut {}", i);
}

for j in 1..10 {         // Exclusif : 1 à 9
    println!("j vaut {}", j);
}
```

**Astuce :** `..=` inclut la valeur finale, `..` l'exclut.

### Itération sur des Tableaux avec enumerate()

```rust
let voitures = ["citroen", "renault", "jeep"];
for (i, voiture) in voitures.iter().enumerate() {
    println!("Index {} : {}", i, voiture);
}
```

**Cas d'usage concret :** Afficher un menu numéroté
```rust
let options = ["Afficher solde", "Retrait", "Liste des comptes", "Quitter"];
for (i, option) in options.iter().enumerate() {
    println!("{}. {}", i + 1, option);  // On commence à 1
}
```

### Autres Types de Boucles

```rust
// Loop infinie (jusqu'au break)
let mut compteur = 0;
loop {
    println!("Compteur: {}", compteur);
    compteur += 1;
    if compteur == 3 {
        break;
    }
}

// While classique  
let mut compteur2 = 0;
while compteur2 < 4 {
    println!("Compteur2 : {}", compteur2);
    compteur2 += 1;
}
```

---

## 4. Tableaux et Vecteurs - Statique vs Dynamique

### Tableaux Fixes
```rust
let voitures = ["citroen", "renault", "jeep"];  // Taille fixe
println!("{}", voitures[1]);  // Accès par index
```

### Vecteurs Dynamiques
```rust
let noms = vec![String::from("Erwan"), String::from("Salma")];
// Taille variable, plus flexible
```

**Différence importante :** Les tableaux ont une taille fixe définie à la compilation, les vecteurs peuvent grandir.

---

## 5. Gestion des Entrées Utilisateur - Le Pattern Match

```rust
use std::io;  // Bibliothèque input/output

println!("Veuillez saisir un numéro");
let mut choix = String::new();
io::stdin().read_line(&mut choix).expect("Erreur de lecture");

// Le pattern matching pour convertir
let choix: usize = match choix.trim().parse() {
    Ok(num) => num,           // Si la conversion réussit
    Err(_) => {               // Si ça échoue
        println!("Numéro invalide");
        return;
    }
};
```

**Pattern matching :** C'est comme un switch/case mais en plus puissant. On gère explicitement les cas d'erreur.

---

## 6. Les Structures - L'Alternative aux Classes

### Concept Fondamental

Rust n'a pas de classes comme Java/C++. À la place, on a des **structures** (struct) pour organiser les données.

```rust
// Définition de la structure
struct Salarie {
    nom: String,
    prenom: String, 
    age: u32
}

// Création d'instances
let salarie1 = Salarie {
    nom: String::from("Harbaoui"),
    prenom: String::from("Aymen"),
    age: 40
};
```

**Philosophie :** Rust sépare les données (struct) des comportements (impl).

### Implémentation de Méthodes avec impl

```rust
impl Salarie {
    fn afficher(&self) {  // &self = lecture seule
        println!("Salarié: {} {} - {} ans", 
                 self.nom, self.prenom, self.age);
    }
}

// Utilisation
salarie1.afficher();
```

---

## 7. Exemple Complet - Compte Bancaire

Voici l'exemple qui résume tout ce qu'on a appris :

```rust
struct CompteBancaire {
    nom: String,
    solde: f64
}

impl CompteBancaire {
    // Lecture seule avec &self
    fn afficher(&self) {
        println!("Compte de {}, solde : {} €", self.nom, self.solde);
    }
    
    // Modification avec &mut self
    fn deposer(&mut self, montant: f64) {
        self.solde += montant;
        println!("+{} € déposés", montant);
    }
    
    fn retirer(&mut self, montant: f64) {
        if self.solde >= montant {
            self.solde -= montant;
            println!("-{} € retirés", montant);
        } else {
            println!("Solde insuffisant !");
        }
    }
    
    fn fermer(&mut self) {
        println!("Compte de {} fermé, dernier solde : {} €", 
                 self.nom, self.solde);
        self.solde = 0.0;
    }
}

fn main() {
    let mut compte = CompteBancaire {
        nom: String::from("Salma"),
        solde: 3000.0
    };
    
    compte.afficher();
    compte.deposer(30.0);
    compte.retirer(20.0);
    compte.afficher();
    compte.fermer();
}
```

### Points Clés sur &self vs &mut self

- `&self` : lecture seule (comme const en C++)
- `&mut self` : modification autorisée
- `self` : consomme l'objet (plus utilisable après)

---

## 8. Les Traits - L'Alternative aux Interfaces

Les traits remplacent les interfaces d'autres langages :

```rust
trait Animal {
    fn chanter(&self);
    fn crier(&self);
}

struct Chien;

impl Animal for Chien {
    fn chanter(&self) {
        println!("wouaf wouaf wouaf !!!");
    }
    
    fn crier(&self) {
        println!("OUAF OUAF !");
    }
}
```

**Comparaison avec Java :**
```java
// Java
interface Animal {
    void chanter();
    void crier();  
}

class Chien implements Animal {
    public void chanter() {
        System.out.println("wouaf wouaf !");
    }
}
```

---

## Concepts Importants à Retenir

### 1. Ownership (Propriété)
Chaque valeur a un propriétaire unique. Quand le propriétaire sort de scope, la mémoire est libérée automatiquement.

### 2. Références et Emprunts  
- `&` : emprunt en lecture seule
- `&mut` : emprunt en écriture

### 3. String vs &str
- `String` : chaîne possédée, modifiable
- `&str` : référence vers une chaîne, non modifiable

### 4. Pattern Matching
Gestion explicite des cas d'erreur avec `match`, `Ok()`, `Err()`.

---

## Ressources pour Aller Plus Loin

- Documentation officielle : https://doc.rust-lang.org/rust-by-example/
- Le prof recommande de pratiquer avec des petits projets
- Rust Book pour approfondir les concepts avancés

---

## Mes Impressions Personnelles

Venant des systèmes/réseaux, Rust me semble être un excellent compromis :
- Performance du C pour les applications système
- Sécurité mémoire sans garbage collector
- Syntaxe plus moderne et lisible
- Gestion d'erreurs explicite (important en réseau !)

Le concept d'ownership est difficile au début, mais on comprend vite l'intérêt : plus de segfaults ni de fuites mémoire.

**Note :** Ce README synthétise 7h de découverte intensive. Rust demande du temps pour être maîtrisé, mais les bases sont solides !