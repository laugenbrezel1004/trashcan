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