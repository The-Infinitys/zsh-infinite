pub fn update() {
    println!("Updating zsh-infinite...");
    crate::utils::install();
    println!("Update complete! Please restart your Zsh session or run 'source ~/.zshrc' to apply the changes.");
}