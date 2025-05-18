# 🚮 trashcan - Because `rm -rf` is the Keyboard Equivalent of a Grenade

![Trashcan Icon - A sassy, overflowing bin](https://via.placeholder.com/150/FF5733/FFFFFF?text=Trashcan+BOOM)

> **"I meant to delete `temp.txt`, not my entire thesis!"** — You, probably, at 3 AM.

Welcome to `trashcan`, a Rust-powered `rm` replacement that saves your files (and your sanity) by moving them to a safe trash directory at `~/.local/share/trashcan` instead of obliterating them into the digital abyss. Think of it as a polite bouncer for your files: they’re not *gone*, just chilling in a VIP lounge with a unique UUID tag until you decide their fate.

**Disclaimer**: This is a learning project. It’s like a toddler with a flamethrower—adorable, but don’t trust it with your production server. Always back up your data, because I’m not your mom. Also, **Linux-only** for now, because penguins rule.

## 🎉 Why `trashcan`?
- **Safety Net for Butterfingers**: Moves files to `~/.local/share/trashcan` instead of yeeting them into oblivion.
- **UUID-Tagged Backups**: Deleted files get a unique UUID suffix (e.g., `thesis.txt:550e8400-e29b-41d4-a716-446655440000`), because timestamps are so 2024.
- **Sassy CLI with Clap**: Built with the `clap` crate for a command-line experience smoother than your grandma’s gravy.
- **User-Specific Trashcans**: Uses the `users` crate to stash files in your home directory, so your trash doesn’t mingle with your roommate’s.
- **Nuke Mode (with a Smirk)**: A `--nuke` flag for when you’re feeling like a digital grim reaper. Use it sparingly, or don’t blame me when you cry.
- **Trashcan Management**: Clear the entire trashcan with `--trashcan` or peek inside with `--show-trashcan`.
- **Tested Like a Boss**: Comprehensive unit tests ensure `trashcan` doesn’t trash your trust.

## 🛠️ Installation: Easier Than Explaining “I Swear It Wasn’t Me” to Your Boss

No `prod.db` disasters here! Installing `trashcan` is so simple, even your cat could do it (if they weren’t busy knocking over your coffee). Choose your adventure: the **Lazy Way** for instant gratification or the **Hero Way** for Rust-fueled glory. **Note**: Linux-only, so Windows and macOS users, hold tight for future updates.

### 🥳 The Lazy Way
Don’t want to wrestle with Rust? Grab the pre-built executable from the [latest release](https://github.com/laugenbrezel1004/trashcan/releases/latest):
1. Download the Linux binary.
2. Move it to a cozy spot:
   ```bash
   sudo mv trashcan /usr/local/bin/
   ```
3. Run `trashcan --version` to confirm it’s alive. Done!

**Warning**: Lazy Way users miss out on Rust’s heartwarming compiler errors. Proceed at your own risk.

### 🦀 The Hero Way (Recommended)
Embrace the Rust lifestyle and build `trashcan` from source. It’s like baking your own cookies—more work, but oh-so-satisfying.

1. **Install Rust**: No Rust, no trust. Get it from [rust-lang.org](https://www.rust-lang.org/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Follow the prompts, then refresh your shell:
   ```bash
   source $HOME/.cargo/env
   ```

2. **Install Dependencies**: You’ll need `libclang` for the `users` crate:
   ```bash
   sudo apt-get install libclang-dev  # Ubuntu/Debian
   sudo dnf install clang-devel      # Fedora
   sudo pacman -S clang              # Arch
   ```

3. **Clone the Repo**:
   ```bash
   git clone https://github.com/laugenbrezel1004/trashcan.git
   cd trashcan
   ```

4. **Build the Magic**:
   ```bash
   cargo build --release
   ```
   This creates a shiny binary in `target/release/trashcan`. Expect some Rust compiler sass—it’s just showing off.

5. **Make It Accessible**:
   Copy the binary to `/usr/local/bin` for global access:
   ```bash
   sudo cp target/release/trashcan /usr/local/bin/
   ```

6. **Optional Pro Move**: Add it to your shell config for permanence (e.g., `.bashrc`, `.zshrc`):
   ```bash
   export PATH=$PATH:/path/to/trashcan/target/release
   ```

**Pro Tip**: If Rust throws a tantrum (weird errors?), try `cargo clean` and rebuild. If that fails, bribe the compiler with `cargo build --release --verbose` and a heartfelt “I believe in you.”

### 🛡️ Troubleshooting
- **“Command not found”**: Ensure `/usr/local/bin` is in your PATH. Run `echo $PATH` to check.
- **Rust version too old?**: Update with `rustup update`.
- **Missing `libclang`?**: Install `libclang-dev` or equivalent for your distro.
- Still stuck? Open an [issue](https://github.com/laugenbrezel1004/trashcan/issues) and I’ll send virtual cookies.

## 🚀 Usage: Delete Like a Pro, Regret Like an Amateur

```bash
trashcan [OPTIONS] <file1> <file2> ...
```

This moves your files to `~/.local/share/trashcan` with a UUID suffix, giving you a chance to rethink your life choices.

### Flags of Glory
- `--nuke`: Skips the trashcan and deletes *permanently*. It’s like `rm -rf` with a villainous laugh. **USE WITH CAUTION.**
- `--trashcan`: Nukes the entire trashcan directory (but recreates it empty). Think of it as a digital spring cleaning.
- `--show-trashcan`: Lists all files in the trashcan. Perfect for reminiscing about your bad decisions.
- `--version`: Shows the current version of `trashcan`.
- `--help`: Displays the help message, because even heroes need a manual.

### Examples
```bash
# Send files to the trashcan
trashcan oops.txt secrets.docx

# Delete multiple files with flair
trashcan photo1.jpg photo2.jpg photo3.jpg

# Go full supervillain (careful!)
trashcan --nuke top_secret_plans.pdf

# Peek into your trashcan
trashcan --show-trashcan

# Clear the trashcan (no turning back)
trashcan --trashcan
```

**Trashcan Location**: Files chill at `~/.local/share/trashcan`. Want them back? For now, manually `mv` them out (e.g., `mv ~/.local/share/trashcan/oops.txt:550e8400-e29b-41d4-a716-446655440000 ~/oops.txt`).

## 🤝 Contributing: Join the Trash Party

Love `trashcan`? Hate it? Either way, help make it better! Fork the repo, open issues, or submit PRs. Here’s how to win my heart:

- **Fix Bugs**: Found a glitch? Report it, and I’ll wrestle the Rust borrow checker to fix it.
- **Add Features**: Got a wild idea? Like a `--laugh` flag that plays a cackle on delete? I’m listening.
- **Write Tests**: Tests are like vegetables—nobody loves them, but we need them.
- **Polish Docs**: Make this README even more legendary.

**How to Contribute**:
1. Fork the repo.
2. Create a branch (`git checkout -b feature/epic-idea`).
3. Commit your changes (`git commit -m "Added --laugh flag for evil vibes"`).
4. Push and open a PR.

**Code of Conduct**: Be kind, like you’re explaining Rust to your grandma. We follow the [Rust Code of Conduct](https://www.rust-lang.org/conduct.html).

## 📜 License
MIT License. Free as in “free pizza,” but you still have to clean up your own messes. See [LICENSE](LICENSE) for details.

## ⚠️ Disclaimer (Yes, Again)
This is a learning project. It’s not your personal data bodyguard. If you delete your wedding photos or your company’s database, don’t send me angry emails. **Always back up your data.**

## 🌌 Future Plans (AKA My Daydreams)
- **Config File**: Customize trashcan location and retention period via a `config.toml`.
- **Restore Command**: Bring files back from the dead with a single command.
- **List Command Enhancements**: Show file sizes, types, or deletion dates with `--show-trashcan`.
- **Autocompletion**: Add shell autocompletion for a silky-smooth CLI experience.
- **Environment Variables**: Support custom trashcan paths via env vars.
- **Cross-Platform Support**: Maybe macOS and Windows, if I can bribe the Rust gods.
- **Permission Checks**: Warn about files you can’t delete due to permissions.

## 🎤 Acknowledgments
- **Laurenz Schmidt**: The original trashcan visionary.
- **Rust Community**: For making me cry and learn at the same time.
- **You**: For reading this far. You’re the real MVP.

## 💬 Final Words
`trashcan` is your friend, but it’s not perfect. It’s like a pet raccoon—cute, clever, but it might still rummage through your files. Use it, love it, but always have a backup plan.

> **"In the end, we only regret the files we didn’t back up."** — Probably not Gandhi

*Built with 🦀 Rust, ☕ coffee, and a pinch of existential dread.*