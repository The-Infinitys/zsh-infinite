# Zsh-Infinite: The Highly Customizable & Dynamic Zsh Theme

![basic](examples/basic/image.png)
![infinite](examples/infinite/image.png)

**Zsh-Infinite** is a powerful and highly customizable Zsh theme, meticulously crafted in Rust for unparalleled performance and flexibility. It transforms your terminal prompt into a dynamic, visually engaging, and informative interface, allowing you to tailor every aspect of its appearance and content.

## ✨ Features

-   **🚀 Blazing Fast**: Engineered in Rust, Zsh-Infinite delivers exceptional speed and responsiveness, ensuring your terminal experience remains fluid.
-   **🎨 Unrivaled Customization**:
    *   **Dynamic Prompt Generation**: Render sophisticated left, right, and even transient prompts with content sourced directly from shell commands.
    *   **Rich Coloring Options**: Define background, foreground, primary, secondary, and accent colors to match your aesthetic.
    *   **Stunning Accent Gradients**: Elevate your prompt's visual appeal with single accent colors, vibrant rainbow gradients, or custom multi-stop gradients.
    *   **Diverse Connection Styles**: Choose from a variety of line styles (e.g., `Line`, `Double`, `Bold`, `Dashed`, `Dotted`) to elegantly connect prompt segments.
    *   **Expressive Segment Separators**: Utilize distinct shapes (e.g., `Sharp` Powerline triangles, `Slash`, `Round`, `Wave`) for clear and visually appealing prompt segmentation.
-   **💡 Intuitive Interactive Configuration**: Effortlessly personalize your prompt's appearance with the `zsh-infinite theme` command, launching an interactive command-line interface.
-   **⚙️ Seamless Management**: Enjoy straightforward installation and uninstallation processes via dedicated CLI commands.
-   **🧪 Developer Friendly**: Includes a `dev` command for testing your theme modifications in an isolated Zsh environment, safeguarding your main configuration.

## ⚠️ Prerequisites

-   **[Rust](https://rustup.rs/)**: Zsh-Infinite is built with Rust. Please ensure you have Rust and Cargo installed.
-   **[Zsh](https://www.zsh.org/)**: Your system must have Zsh installed as its shell.

## 💻 Installation

Install Zsh-Infinite directly from source:

```bash
cargo install --git https://github.com/The-Infinitys/zsh-infinite zsh-infinite
```

After installation, set up the theme in your Zsh configuration:

```bash
zsh-infinite install
```

The `install` command performs the following actions:

1.  Copies the `zsh-infinite` executable to your Cargo bin directory.
2.  Creates an `infinite.zsh-theme` file in your Zsh configuration.
3.  Generates a snippet file in `~/.config/zsh-infinite` to properly source the theme.
4.  Modifies your `~/.zshrc` to automatically load the generated theme snippet.

**Important**: For the changes to take effect, please restart your Zsh session or run `source ~/.zshrc`.

## 🚀 Usage

Once installed, the `infinite` Zsh theme will be automatically activated.

### Interactive Theme Configuration

To customize your prompt's visual style and content, use the interactive configuration utility:

```bash
zsh-infinite theme
```

This command will launch a user-friendly menu allowing you to adjust colors, segment styles, separators, and more.

### CLI Commands

The `zsh-infinite` command-line interface provides comprehensive tools for managing your theme:

-   `zsh-infinite install`: Installs the theme and the CLI tools, integrating them with your Zsh setup.
-   `zsh-infinite uninstall`: Safely removes the theme and CLI, reverting any modifications to your `~/.zshrc`.
-   `zsh-infinite theme`: Opens the interactive UI for real-time theme customization.
-   `zsh-infinite dev`: (For Developers) Initiates a clean, sandboxed Zsh session for testing theme changes without impacting your primary Zsh configuration.

For a complete list of commands and their options, execute:

```bash
zsh-infinite --help
```

## 📝 Configuration

Your theme's settings are stored in `~/.config/zsh-infinite/theme.yaml`. While you can edit this file manually, using the `zsh-infinite theme` interactive command is the recommended and easiest way to manage your configuration.

The `theme.yaml` allows for fine-grained control over:

-   **Colors**: Define intricate color schemes, including support for complex accent color gradients across different prompt elements.
-   **Connection**: Select various character styles (e.g., single line, double line, bold, dashed) for visual continuity between prompt segments.
-   **Separators**: Choose distinct separator styles (e.g., Powerline-style triangles, slashes, rounded edges) for the beginning, middle, and end of both left and right prompt segments.

### Example `theme.yaml`

```yaml
prompt_contents_list:
  - left:
      - cmd: zsh
        args:
          - -c
          - whoami
      - cmd: zsh
        args:
          - -c
          - hostname
    right:
      - cmd: zsh
        args:
          - -c
          - echo ${PWD/#$HOME/\~}
      - cmd: zsh
        args:
          - -c
          - echo $LAST_STATUS
    color:
      bg: Black
      fg: White
      pc: Cyan
      sc: LightBlack
      accent: !single LightBlack
      accent_which: ForeGround
    connection: None
    left_segment_separators:
      start_separator: Sharp
      mid_separator: Sharp
      end_separator: Sharp
      edge_cap: true
      bold_separation: true
    right_segment_separators:
      start_separator: Sharp
      mid_separator: Sharp
      end_separator: Sharp
      edge_cap: true
      bold_separation: true
    accent_which: ForeGround
transient_color:
  bg: Black
  fg: White
  pc: Cyan
  sc: LightBlack
  accent: !single LightBlack
  accent_which: ForeGround
```

Discover more configuration examples in the [examples directory](./examples/).

## 🧑‍💻 Development

### Building from Source

To build `zsh-infinite` from its source code:

```bash
git clone https://github.com/the-infinitys/zsh.theme.infinite.git
cd zsh.theme.infinite
cargo build
```

### Testing with `zsh-infinite dev`

To test your modifications in an isolated Zsh environment without impacting your primary configuration:

```bash
cargo run -- dev
```

This command conveniently sets up a temporary `run` directory with a minimal `.zshrc` and `.zsh-theme` that sources your development build of `zsh-infinite`, then launches a new Zsh session for testing.

## 🤝 Contributing

Contributions are highly valued! Whether it's bug reports, feature suggestions, or pull requests, your involvement helps make Zsh-Infinite even better. Please feel free to [open an issue](https://github.com/The-Infinitys/zsh-infinite/issues) or submit a [pull request](https://github.com/The-Infinitys/zsh-infinite/pulls).

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for full details.