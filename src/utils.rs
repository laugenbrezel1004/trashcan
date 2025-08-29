pub fn vprint(message: String, verbose: bool) {
    if verbose {
        print!("{}", message);
    }
}
