# Lockkey

Lockkey is simple secrets manager that allows you to store passwords and text.

# Features

https://github.com/user-attachments/assets/de46bea8-17d3-4c4f-9fe8-5983921b2862

- Securely stores your passwords or text on disk using robust encryption.
- Includes a generate random password feature for creating strong passwords effortlessly.
- Allows you to view your stored passwords or copy them to your clipboard at any time.
- Automatically signs the user out after a period of inactivity, ensuring your data remains secure.

# Encryption

Lockkey uses Argon2 to derive a key from your master password and AES-GCM to encrypt your data before storing it on disk.

# Releases

Releases can be found at the [releases page](https://github.com/rmarinn/lockkey/releases). Alternatively you can [build the project](#building-from-source) on your own.

# Building from source

To build Lockkey from source, clone the repository and use either npm or pnpm:

```bash
git clone https://github.com/rmarinn/lockkey.git
cd lockkey
npm install
npm run tauri build
```

the build should be in `/src-tauri/target/release`

# License

Lockkey Licensed under the [MIT License](./LICENSE)
