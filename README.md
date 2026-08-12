# 🛒 Simulation de Serveurs de Commandes Concurrents (Rust)

Un programme écrit en **Rust** illustrant la gestion de la concurrence et du parallélisme. Il simule un système de traitement de commandes où un groupe de **workers (serveurs) en arrière-plan** traitent des commandes saisies en direct dans le terminal, tout en partageant un **stock commun** de manière sécurisée.

---

## 💡 Fonctionnalités & Concepts Rust illustrés

Ce projet est un exemple concret de l'utilisation des primitives de concurrence de Rust :

* **`std::thread`** : Création de 5 threads serveurs autonomes exécutés en arrière-plan.
* **`mpsc::channel`** : Canal de communication (*Multiple Producer, Single Consumer*) permettant d'envoyer des commandes depuis l'entrée terminal (`tx`) vers la file d'attente.
* **`Arc<Mutex<T>>`** : 
  * **`Arc`** (*Atomically Reference Counted*) pour partager la propriété de la file d'attente et du stock entre plusieurs threads sans violation d'Ownership.
  * **`Mutex`** (*Mutual Exclusion*) pour garantir un accès exclusif en écriture au stock partagé et à la réception des commandes.
* **Gestion interactive (CLI)** : Saisie dynamique de la durée et de la quantité de chaque commande via le terminal (`std::io::stdin`).

---

## 🛠️ Prérequis

Pour pouvoir compiler et exécuter ce projet, vous devez avoir **Rust** et **Cargo** (le gestionnaire de paquets et de build officiel) installés sur votre machine.

### Installation de Rust

* **Linux / macOS :**
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
  source "$HOME/.cargo/env"
  rustc main.rs


###  📖 Utilisation
Une fois le programme main lancé, 5 threads serveurs démarreront en arrière-plan. Vous pourrez saisir vos commandes directement dans le terminal.
(dans le code, le nombre de thread est de 5 et le stock est de 100.
On peut modifier ces valeurs en les changeant dans les lignes avec le commentaire //modifiable)

Format des commandes :
Saisissez la durée du traitement (en secondes) suivie de la quantité d'articles à retirer, séparées par un espace :
