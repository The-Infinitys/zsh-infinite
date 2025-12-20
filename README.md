# Infinite Zsh Theme

A highly customizable and dynamic Zsh theme written in Rust. Infinite offers a powerful CLI tool to manage your Zsh prompt's appearance, allowing for dynamic content, sophisticated coloring, and unique visual separators.

## Features

*   **Dynamic Zsh Prompt Generation**: Renders left, right, and transient prompts with content sourced from shell commands.
*   **Highly Customizable Visuals**:
    *   **Colors**: Define background, foreground, primary, secondary, and accent colors.
    *   **Accent Colors**: Choose from single colors, rainbow gradients, or custom multi-stop gradients for a vibrant look.
    *   **Connection Styles**: Select various line styles (e.g., `Line`, `Double`, `Bold`, `Dashed`, `Dotted`) for prompt segmentation.
    *   **Segment Separators**: Utilize different shapes (e.g., `Sharp` Powerline triangles, `Slash`, `Round`, `Wave`) to visually separate prompt segments.
*   **Interactive Theme Configuration**: Use the `zsh-infinite theme` command to easily configure your prompt's appearance through an interactive CLI.
*   **Easy Installation & Uninstallation**: A dedicated CLI provides commands to set up and remove the theme from your system.
*   **Development Utilities**: Includes a `dev` command to test the theme in a clean Zsh environment.

## Installation

### Prerequisites

*   **Rust**: You need to have Rust and Cargo installed. If not, follow the instructions on [rustup.rs](https://rustup.rs/).
*   **Zsh**: Ensure Zsh is installed on your system.

### Install via Cargo (Recommended)

Once the package is published to crates.io, you will be able to install it using:

```bash
cargo install zsh-infinite
```

### Install from Source

```bash
git clone https://github.com/the-infinitys/zsh.theme.infinite.git
cd zsh.theme.infinite
cargo build --release
./target/release/zsh-infinite install
```

The `install` command will:
1. Copy the `zsh-infinite` executable to `~/.config/zsh-infinite/bin`.
2. Create an `infinite.zsh-theme` file in `~/.config/zsh-infinite`.
3. Generate a snippet file in `~/.config/zsh-infinite` to source the theme.
4. Modify your `~/.zshrc` to source the generated snippet.

**Important**: After installation, restart your Zsh session or run `source ~/.zshrc` to apply the changes.

## Usage

The `infinite` Zsh theme will automatically activate after installation.

### Interactive Theme Configuration

To customize your prompt, use the interactive configuration tool:

```bash
zsh-infinite theme
```

This will open a menu where you can:
*   Add/Remove prompt lines.
*   Configure colors (background, foreground, primary, secondary, and accent colors).
*   Choose connection styles between prompt segments.
*   Select separator styles for left and right prompt segments.

### CLI Commands

The `zsh-infinite` CLI provides the following commands:

*   `zsh-infinite zsh prompt left`: Generates the left-hand side of the prompt.
*   `zsh-infinite zsh prompt right`: Generates the right-hand side of the prompt.
*   `zsh-infinite zsh prompt transient [--exit-code <CODE>]`: Generates the transient prompt after command execution.
*   `zsh-infinite update`: Updates the `zsh-infinite` application to the latest version.
*   `zsh-infinite install`: Installs the theme and CLI (as described above).
*   `zsh-infinite uninstall`: Uninstalls the theme and CLI, reverting changes to `~/.zshrc`.
*   `zsh-infinite theme`: Launches the interactive theme configuration UI.
*   `zsh-infinite dev`: (Debug only) Starts a clean Zsh session for development and testing.

## Configuration

Theme configurations are stored in `~/.config/zsh-infinite/theme.yaml`. You can modify this file directly, but it's recommended to use the `zsh-infinite theme` interactive command for easier management.

The configuration allows detailed control over:
*   **Colors**: Define colors for various parts of the prompt, including complex accent color gradients.
*   **Connection**: Select from predefined connection characters (e.g., single line, double line, bold, dashed) for visual continuity.
*   **Separators**: Choose distinct separator styles (e.g., Powerline-style triangles, slashes, rounded edges) for the beginning, middle, and end of prompt segments on both the left and right sides.

## Development

### Building from Source

```bash
git clone https://github.com/the-infinitys/zsh.theme.infinite.git
cd zsh.theme.infinite
cargo build
```

### Testing with `zsh-infinite dev`

To test your changes in a sandboxed Zsh environment without affecting your main Zsh configuration:

```bash
cargo run -- dev
```

This command will set up a temporary `run` directory with a minimal `.zshrc` and `.zsh-theme` that sources your development build of `zsh-infinite`, then launches a new Zsh session.

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.