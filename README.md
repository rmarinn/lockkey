# Lockkey

Lockkey is simple secrets manager that allows you to store passwords and text.

# Features

- will securely store password or text in the disk
- has a *generate random password* feature
- you can also view you password or copy it to your clipboard any time
- lockkey will automatically sign the user out if it does not detect any activity for an amout of time

# Encryption

Lockkey uses argon2 to derive a key from your master password and uses AES-GCM to encrypt your data before storing it onto the disk.

# Releases

Releases can be found at the [releases page](https://github.com/rmarinn/lockkey/releases) or you can [build the project](#building-from-source) on your own.

# Building from source

simply clone the repo and use npm or pnpm (whichever you prefer)

```
npm install
npm run tauri build
```

the build should be in `/src-tauri/target/release`

# License

Lockkey Licensed under the [MIT License](https://github.com/rmarinn/lockkey/blob/main/LICENSE)
