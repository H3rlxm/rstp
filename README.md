# 🔑 RsOTP

**Authenticator TOTP desktop — Rust + egui + SQLite**

Application de gestion de codes TOTP (comme Google Authenticator / FreeOTP) en Rust avec interface graphique native.

---

## ✨ Fonctionnalités

| Fonctionnalité | Description |
|---|---|
| 🔐 **TOTP** | Génération RFC 6238 (HMAC-SHA1, 6 chiffres, 30s) |
| 🖼️ **Scan QR** | Déposer une image PNG/JPG → QR décodé automatiquement |
| 📋 **Copie rapide** | Cliquer sur un code → copié dans le presse-papier |
| 💾 **Stockage** | SQLite dans `~/.config/rsotp/rsotp-codes.db` |
| 🗑️ **Gestion** | Ajout manuel, suppression, auto-sauvegarde |
| 📊 **Barre progression** | Visuel 30s, rouge < 5s |

---

## 📸 Captures d'écran

| Ajout de compte | Liste des codes |
|---|---|
| ![Ajout](Image%20coll%C3%A9e.png) | ![Liste](Image%20coll%C3%A9e%20(2).png) |

---

## 🚀 Installation

```bash
git clone https://github.com/votre-utilisateur/rsotp.git
cd rsotp
cargo run --release
```

### Prérequis
- **Rust** 1.75+ ([rustup.rs](https://rustup.rs))
- **Système** : Linux, macOS, Windows

---

## 🖥️ Utilisation

### Ajouter un compte
1. Cliquer sur **+**
2. Entrer le **Label** (ex: `Google`)
3. Entrer le **Secret** (clé Base32, ex: `JBSWY3DPEHPK3PXP`)
4. Cliquer **Save**

Ou déposer une capture d'écran / image contenant un QR code.

### Copier un code
Cliquer sur le code à 6 chiffres → copié dans le presse-papier.

### Supprimer un compte
Cliquer sur le bouton **x** à droite du code.

---

## 📂 Structure

```
rsotp/
├── Cargo.toml
└── src/
    ├── main.rs      # GUI egui (liste codes, drag&drop QR)
    ├── db.rs        # SQLite CRUD
    ├── totp.rs      # Génération TOTP (HMAC-SHA1)
    └── qr.rs        # Scan QR code depuis image
```

### Dépendances

| Crate | Utilisation |
|---|---|
| `eframe` / `egui` | Interface graphique |
| `rusqlite` | Base SQLite |
| `arboard` | Presse-papier |
| `hmac` + `sha1` | HMAC-SHA1 pour TOTP |
| `base32` | Décodage clés Base32 |
| `rqrr` + `image` | Décodage QR code |

---

## 🔒 Sécurité

- Les clés sont stockées localement en clair dans SQLite (`~/.config/rsotp/`)
- Aucune connexion réseau, aucun serveur
- Les codes sont générés localement via HMAC-SHA1

---

## 📄 Licence

MIT

---

*construit avec [Rust](https://www.rust-lang.org/) 🦀*
