# Hasp 🔒

**Hasp** is a secure, local-first command-line password manager built in Rust. It emphasizes security best practices, type safety, and ergonomic user experience.

## 🚀 Project Goals

- **Secure by Default**: Uses AES-GCM encryption and Argon2id key derivation.
- **Local Storage**: Data is stored in a local SQLite database, giving the user full control.
- **Zero-Knowledge**: The master password is never stored; it is used to derive the encryption key.
- **Memory Safety**: Sensitive data is zeroized (wiped) from memory when dropped.

## 🛠 Architecture

This project is structured as a Rust Workspace:

- **`hasp_core`**: The library containing domain models, cryptographic logic, and database interactions.
- **`hasp_cli`**: The binary handling user input, command parsing (Clap), and clipboard integration.

## 🗺️ Roadmap

### Phase 1: The Foundation (Core) 🚧

- [ ] Define Domain Models (Service, Username, Encrypted Password).
- [ ] Implement Custom Error Handling (`thiserror`).
- [ ] Set up SQLite Database connection and schema creation.
- [ ] Implement CRUD operations (Add, List, Delete).

### Phase 2: Security (Crypto) 🔐

- [ ] Implement Argon2id for Master Password hashing.
- [ ] Implement AES-GCM for payload encryption/decryption.
- [ ] Integrate `zeroize` to scrub memory.

### Phase 3: The Interface (CLI) 💻

- [ ] Set up `clap` for subcommands (Init, Add, Get, List).
- [ ] Implement hidden password input.
- [ ] Connect CLI commands to Core logic.

### Phase 4: User Experience (UX) ✨

- [ ] Add Clipboard support (auto-clear after 15s).
- [ ] Add search functionality.
- [ ] Add import/export (JSON).

## 📦 usage

```bash
cargo run --bin hasp_cli -- add --service google --username myuser
cargo run --bin hasp_cli -- get google
```
