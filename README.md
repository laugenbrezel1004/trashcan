# trashcan - Because `rm -rf` is just too mainstream

![trashcan icon - maybe a cute overflowing bin?](https://via.placeholder.com/150/FF0000/FFFFFF?text=Trashcan+Icon)

## What is this?

Let's be honest. You've accidentally `rm -rf`'d something important. We've *all* been there.  `trashcan` is a rust implementation of a slightly-less-dangerous `rm`. Instead of immediately deleting files into the void, it moves them to a temporary "trashcan" directory.  Think of it as a safety net for your clumsiness.  It's basically `rm` but with a really polite (and slightly condescending) attitude.

**Disclaimer:** This is a learning project. Don't rely on it for critical data!  I'm still learning Rust, and honestly, I'm mostly here for the compiler errors.

## Features

*   **Moves files to a trashcan:**  Instead of deleting, it `mv`s to a directory, giving you a chance to recover (before the trashcan overflows).
*   **Configuration (eventually):** Future plans include a configuration file to customize the trashcan location and retention period.  We'll get there... eventually.
*   **Debugging goodness:** Built with `debug_assertions` in mind!  Lots of `println!` statements to help you understand what's going on (and to keep me entertained).
*   **Forced deletion (use with extreme caution):** A `--force` flag for when you *really* mean it.  Seriously, be careful with this one. We are not responsible for data loss.
*   **Clap for Argument Parsing:** Uses the `clap` crate to make command-line arguments... less painful.
*   **Nix for User ID:**  Leverages `nix` to get the user ID, ensuring each user has their own trashcan (so you don't accidentally delete your roommate's files... or do, if you're into that kind of thing).

## Installation

1.  **Make sure you have Rust installed:** If not, head to [https://www.rust-lang.org/](https://www.rust-lang.org/) and follow the instructions.

2.  **Clone the repository:**

    ```bash
    git clone https://github.com/your-username/trashcan.git
    cd trashcan
    ```

3.  **Build the project:**

    ```bash
    cargo build --release
    ```

4.  **Add the executable to your PATH:**  (This step depends on your operating system.)

    *   **Linux/macOS:**

        ```bash
        export PATH=$PATH:./target/release
        ```

        Add this line to your `.bashrc` or `.zshrc` file to make it permanent.

    *   **Windows:**  You'll need to manually add the `target\release` directory to your PATH environment variable.

## Usage

```bash
trashcan <file1> <file2> ...
```

This will move the specified files to the trashcan directory (usually `/tmp/trashcan-<user_id>`).

**Flags:**

*   `--force <file>`:  Deletes the file immediately, bypassing the trashcan. Use with extreme caution!  (Seriously, I'm warning you.)
*   `-s` or `--show-config`: Show the current config file. (Currently doesn't do much, but it will... eventually.)

**Example:**

```bash
trashcan my_important_document.txt another_file.pdf
trashcan --force dangerous_file.txt  #Be very careful with this!
```

## Contributing

Feel free to open issues and pull requests!  I'm still learning, so any help is appreciated.

**Here's how you can help:**

*   **Fix bugs:** If you find a bug, please open an issue and I'll try to fix it.
*   **Add features:**  Got a great idea for a new feature?  Open a pull request!
*   **Improve documentation:**  The documentation is a work in progress.  Help me make it better!
*   **Write tests:**  Tests are always welcome!

## License

This project is licensed under the nothing so far...

## Disclaimer (again!)

I cannot stress this enough: this is a learning project.  Do not rely on it for critical data!  I am not responsible for any data loss that may occur. Use at your own risk!  And please, don't blame me if you accidentally delete something important. You've been warned!

## Future Plans (maybe...)

*   Implement a configuration file to customize the trashcan location and retention period.
*   Add a command to list the files in the trashcan.
*   Add a command to restore files from the trashcan.
*   Add a command to permanently delete files from the trashcan.
*   Add support for recursive deletion (be careful with this one!).
*   Maybe add a GUI? (Okay, that's probably a bit ambitious.)

Written by some local AI with https://github.com/laugenbrezel1004/kailian




# 🚮trashcan: The Safer `rm` Command for People Who Hate Regret

**A Rust-based `rm` replacement that moves files to a magical "trash can" instead of vaporizing them like a nuclear explosion.**

---

## 💥 Why Use This?
Because `rm -rf /` is *not* the best idea, especially when you're sleep-deprived and your keyboard is covered in coffee.

This project gives you the thrill of deleting files *without* the anxiety of permanent deletion. It's like `rm`, but with a safety net, a timestamp, and a side of sarcasm.

---

## 🧠 Features
- **No More Regret**: Files go to `/tmp/trashcan-<your-UID>` instead of vanishing into the void.
- **Timestamped Tombstones**: Every deleted file gets a time-stamped suffix (e.g., `file.txt 14:23:01`).
- **`--force` Flag**: For when you're *100% sure* you want to destroy something forever (and you miss `rm -rf`).
- **Clap-powered CLI**: Because `rm`'s help message is as helpful as a screen door on a submarine.

---

## 🛠️ Installation
Just because it's in Rust doesn't mean it's complicated (unless you try to read the docs).

```bash  
cargo build --release  
sudo cp target/release/trashcan /usr/local/bin/  
```  

> ⚠️ Warning: You might need to `sudo` because Rust thinks it's 2003.

---

## 🧪 Usage Examples

```bash  
# Delete a file with style  
trashcan my_important_file.txt  

# Delete multiple files like a boss  
trashcan file1.txt file2.jpg file3.pdf  

# Permanently destroy something (not recommended)  
trashcan --force my_regrets.txt  
```  

> 💣 Pro Tip: `trashcan --force` is just `rm` in disguise. Use it sparingly.

---

## 🧹 Trash Can Location
All deleted files go to:
```
/tmp/trashcan-<your-UID>  
```  
You can fish them out manually if you're desperate.

---

## 🤝 Contributing
1. Fork it.
2. Write code that doesn't make this README look like a lie.
3. Submit a PR.

> 🤡 Bonus points if your contribution includes:
> - A `--laugh` flag that plays a sound when a file is deleted.
> - A `--undo` command to resurrect files.

---

## 📜 License
MIT License (because open-source is free, but your sanity is not).

---

## 🐱 Final Words
**Remember:** Even with this tool, your computer is not a cat. It won't bring back your deleted files just because you say "meow." Always back up your data.

> "The only thing worse than deleting a file is deleting it forever." — Unknown (probably not you, now)

---  

*Made with 🦀 Rust and 100% less regret.*


# Trashcan - A Safer Alternative to `rm`
=====================================

[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://travis-ci.com/trashcan/trashcan.rs.svg?branch=main)](https://travis-ci.com/trashcan/trashcan.rs)

Are you tired of accidentally deleting important files with `rm`? Look no further! Trashcan is a safer alternative that moves deleted files to a temporary trash can directory, allowing you to recover them if needed.

## Features

* Moves deleted files to a temporary trash can directory
* Allows for recovery of deleted files
* Optional force delete feature (use with caution!)
* Configurable duration for keeping deleted files in the trash can

## Installation

To install Trashcan, simply clone this repository and run `cargo build`:
```bash
git clone https://github.com/trashcan/trashcan.rs.git
cd trashcan.rs
cargo build --release
```
Then, add the resulting binary to your system's PATH.

## Usage

Trashcan can be used as a drop-in replacement for `rm`. Simply run `trashcan` followed by the files you want to delete:
```bash
trashcan file1.txt file2.txt
```
By default, Trashcan will move the deleted files to a temporary trash can directory. If you want to force delete the files instead, use the `--force` flag:
```bash
trashcan --force file1.txt file2.txt
```
To view the current config file, use the `--show-config` flag:
```bash
trashcan --show-config
```
## Configuration

Trashcan uses a simple configuration system to determine where to store deleted files and for how long. You can configure these settings by creating a `config.toml` file in your home directory.

### Example Config File
```toml
[trashcan]
location = "/tmp/trashcan"
duration = 10
```
In this example, Trashcan will store deleted files in the `/tmp/trashcan` directory and keep them for 10 minutes.

## Contributing

We welcome contributions to Trashcan! If you have a bug fix or feature request, please open an issue on our GitHub page. If you'd like to contribute code, please fork this repository and submit a pull request.

### Code of Conduct

We follow the Rust community's [code of conduct](https://www.rust-lang.org/conduct.html). Please be respectful and considerate in your interactions with others.

## License

Trashcan is licensed under the MIT license. See [LICENSE](LICENSE) for details.

## Acknowledgments

Thanks to the following contributors:

* Laurenz Schmidt (author)
* [Your Name] ( contributor)

We hope you find Trashcan useful! If you have any questions or need help, don't hesitate to reach out.
