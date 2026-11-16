# Crypt

A minimal tui secure KV manager written in Rust

Crypt is designed to be a simple encrypted KV store for general credentials. The goal is to keep the experience convenient, interface small and reply on local device security instead of cloud services.

> [!IMPORTANT]
> **Project status: design / early development.** The features, commands, and file format described below are targets, not a claim that they are currently implemented or audited. Do not store your only copy of real credentials in this project until it has been thoroughly tested and reviewed.

## Goals

- **Local-first:** one encrypted vault file stored on your device; no backend or telemetry.
- **Simple CRUD:** add, search, update, copy, and delete entries.
- **Master-password unlock:** a single passphrase can recover the vault independently of a specific device.
- **Optional Touch ID:** use macOS Keychain to enable a convenient device-bound unlock method.
- **Authenticated encryption:** detect tampering as well as protect confidentiality.
- **Small, inspectable codebase:** prefer well-maintained cryptographic libraries over custom cryptographic algorithms.

## Intended interface

Since the "key" can be plaintext UTF strings, the actual key will be an ID with separate one to many KV pairs.

```json
{
  "entries": {
    "<openssl hex 16>": {
        "key"": "github",
        "username": "user@example.com",
        "password": "password",
        "notes": "Personal account"
    }
  }
}
```

This is an example of the decrypted loaded representation and not a plaintext file written to disk.

### Issues

- need to find some hash function to make this map searchable well.

## Security design

### Encryption at rest

This is an example security design for us to build off of.

1. Generate a random 256-bit **data encryption key (DEK)** when creating a vault.
2. Serialize the entries in memory.
3. Encrypt the serialized data with **XChaCha20-Poly1305**, using a fresh, random 24-byte nonce for each encryption.
4. Store only the ciphertext and the metadata needed to unlock and decrypt it.
5. Authenticate the relevant vault header and encryption metadata as **associated data (AAD)**.

The DEK is independent of the master password so the password can be changed by rewrapping the DEK rather than re-encrypting all entries.

### Master password

- Derive a **key-encryption key (KEK)** from the master password using **Argon2id**, a unique random salt, and stored KDF parameters.
- Use an authenticated-encryption operation to wrap the DEK under this KEK, with its own fresh nonce.
- On unlock, derive the KEK again, authenticate and unwrap the DEK, then decrypt the vault.
- Calibrate Argon2id parameters for the target machine; record exact parameters in the vault header for future compatibility.

The master password is never stored. A lost master password cannot be recovered from the vault file alone.
There should be a recovery option but not sure right now what it would be

### Optional macOS Touch ID

Touch ID does **not** produce an encryption key from a fingerprint. Instead, macOS Keychain can protect access to a device-local copy of the DEK.

After a successful master-password unlock, the user may opt in to saving the DEK in Keychain with an appropriate access-control policy. Subsequent unlocks can request authentication through Apple's Security and LocalAuthentication frameworks and retrieve the protected DEK.

The master-password recovery path remains available without Touch ID or the original Mac. A Keychain item should be created with explicit access controls and an appropriate device-only accessibility setting; authentication policies must be selected deliberately because some allow the device password as a fallback.

### Proposed vault file

A versioned binary envelope, conceptually:

```text
magic | format_version | KDF parameters | KDF salt
      | password-wrapped DEK + wrap nonce
      | vault nonce | encrypted entries + authentication tag
```

Keychain integration stores a separate protected copy of the DEK **outside** this portable vault file. The final format must specify canonical encoding, size limits, and precisely which fields are authenticated by each encryption operation.

The format is **not finalized**. Avoid producing permanent vault files until migration and backward-compatibility rules are defined.

## Planned Rust dependencies

| Purpose                             | Library / API                                                                                   |
| ----------------------------------- | ----------------------------------------------------------------------------------------------- |
| Password-based key derivation       | [`argon2`](https://crates.io/crates/argon2)                                                     |
| Authenticated encryption            | [`chacha20poly1305`](https://crates.io/crates/chacha20poly1305)                                 |
| Data serialization                  | [`serde`](https://crates.io/crates/serde) + [`serde_json`](https://crates.io/crates/serde_json) |
| Cryptographically secure randomness | An OS-backed CSPRNG via the crypto library / `getrandom`                                        |
| Minimize secret lifetime in memory  | [`zeroize`](https://crates.io/crates/zeroize)                                                   |
| Hidden CLI password prompts         | [`rpassword`](https://crates.io/crates/rpassword)                                               |
| macOS biometric-gated access        | Apple Security and LocalAuthentication frameworks                                               |
| Optional GUI                        | ratatui                                                                                         |

Dependencies and versions will be pinned as implementation begins.

## Proposed CLI

The following is a **planned interface**, not currently guaranteed to work:

```sh
vault init
vault unlock
vault add github
vault list
vault get github
vault edit github
vault delete github
vault lock
```

Commands that reveal or copy secrets should require an unlocked vault. The CLI must not accept master passwords or entry passwords as command-line arguments, where they may leak through process lists or shell history.

## Threat model

**Designed to help protect against:**

- Someone obtaining a copy of the locked vault file.
- Offline password guessing, subject to master-password strength and Argon2id cost.
- Undetected modifications to encrypted vault data.

**Not designed to fully protect against (for now):**

- Malware, keyloggers, or an attacker controlling the machine while the vault is unlocked.
- Secrets captured from application memory, swap, screenshots, or clipboard history.
- Loss of the master password and all viable backups.
- Denial of service, deletion, or rollback to an older valid encrypted vault file.

Full-disk encryption (FileVault), OS updates, and device security are recommended complementary protections.

## Implementation safeguards

- Generate fresh nonces using a cryptographically secure random source; **never reuse a nonce with the same key**.
- Avoid plaintext vault files, debug prints, crash reports, and logs containing credentials.
- Restrict file permissions and protect temporary files.
- Save through a securely created temporary file in the same directory, flush/sync as appropriate, and atomically replace the existing vault; preserve a recoverable encrypted backup.
- Zeroize sensitive buffers where practical, while recognizing that copies and OS-managed memory may remain.
- Minimize clipboard exposure; never assume a clipboard clear timer can defeat clipboard managers.
- Lock on explicit request and after a configurable period of inactivity; consider sleep/lock events for a GUI.
- Authenticate file headers and cap all decoded sizes before allocation.
- Reject corrupted files, incorrect passwords, and unsupported versions safely.
- Test interrupted writes, backup recovery, password changes, Keychain failures, and vault migrations.

## Roadmap

- [ ] Begin with basic KV storage and UI
- [ ] Define and document the versioned vault format and threat model.
- [ ] Implement vault creation, Argon2id key derivation, and DEK wrapping.
- [ ] Implement encrypted save/load with authenticated metadata.
- [ ] Implement CLI CRUD operations and secure password prompting.
- [ ] Add tests for wrong passwords, tampering, invalid input, and nonce handling.
- [ ] Add macOS Keychain + Touch ID unlock.
- [ ] Implement atomic saves, encrypted backups, and recovery tests.
- [ ] Add auto-lock and safer clipboard behavior.
- [ ] Polish UI

## Building

This repository is intended to use the stable Rust toolchain. Once the project is initialized:

```sh
cargo build
cargo test
cargo clippy --all-targets --all-features
```

These are standard development commands, not a claim that a runnable implementation is already present.

## Contributions and security reports

Security-sensitive changes should include tests and an explanation of their impact on the file format, key management, and threat model. Please avoid posting real passwords, decrypted vaults, or sensitive logs in issues or pull requests.

## Disclaimer

This is an experimental password-manager project, **not an audited security product**. Use an established password manager for critical credentials until Crypt has been implemented, tested, and independently reviewed.

## License
