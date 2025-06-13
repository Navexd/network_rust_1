# rust-log-api

## 🚀 Présentation

Cette API écrite en Rust permet de centraliser et d’exposer les logs générés par un serveur proxy (Squid/Apache/Nginx) via des endpoints sécurisés. Elle a été conçue pour être rapide, légère et facilement intégrée à une stack existante.

## 🛠️ Technologies utilisées

- 🔧 Rust (axum = "0.6")
- 🗃️ Fichiers `.log` ou parsing JSON
- 📦 Cargo (gestion de paquets)
- 🐧 Systemd pour exécuter l'API comme service

## ⚙️ Fonctionnalités principales

- 🔐 Authentification simple par token
- 📑 Endpoints REST pour accéder aux logs
- 🧼 Nettoyage automatique des logs anciens
- 📁 Configuration flexible via fichier `.env` ou `config.toml`

## 📦 Installation / Exécution

```bash
# Cloner le repo
git clone https://github.com/Navexd/rust-log-api.git
cd rust-log-api

# Compiler le projet
cargo build --release

# Lancer en local
./target/release/rust-log-api
