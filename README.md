# Simple Password Manager, written in rust
A toy project to learn rust.  
Default password file location is `/usr/local/.spm_rs_passwords`  
Passwords are stored as plain text and protected only by the user's access rights

### Build
`cargo build` - the executable can be found in the `target/debug` directory

### Usage

```bash
$ rs-password-manager help
expected commands:
 help             - print help message
 set <KEY> [PASS] - set a PASS by KEY. PASS can be provided with pipe or interactively
 del <KEY>        - delete entry by KEY
 gen <KEY>        - generate entry for KEY
 get <KEY>        - copy a password to clipboard
 list             - list available entries
```
