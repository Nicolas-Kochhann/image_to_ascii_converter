# Image to ASCII Converter

A Rust-based command-line application that converts images to ASCII art representations.

## Features

- Converts any image format supported by the `image` crate to ASCII art
- Automatic grayscale conversion
- Image resizing with aspect ratio preservation
- Customizable output dimensions
- 69-character gradient for brightness representation

## Installation

### Option 1: Download Binary Release
1. Go to the [Releases page](https://github.com/Nicolas-Kochhann/image_to_ascii_converter/releases)
2. Download the binary:
   - **Linux**: `image_to_ascii_converter`
3. Extract the binary if it's in a compressed archive

### Option 2: Build from Source
1. Clone the repository:
   ```bash
   git clone https://github.com/Nicolas-Kochhann/image_to_ascii_converter.git
   cd image_to_ascii_converter
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Adding to PATH

### Linux
1. Open your shell configuration file:
   - Bash: `~/.bashrc` or `~/.bash_profile`
   - Zsh: `~/.zshrc`
   - Fish: `~/.config/fish/config.fish`
2. Add this line (replace `/path/to/binary` with the actual path):
   ```bash
   export PATH="/path/to/binary:$PATH"
   ```
3. Save the file and reload your shell:
   - Bash/Zsh: `source ~/.bashrc` or `source ~/.zshrc`
   - Fish: `source ~/.config/fish/config.fish`
4. Verify the installation:
   ```bash
   image_to_ascii_converter --help
   ```

## Usage

Run the application with an image path as argument:

```bash
image_to_ascii_converter path/to/image.png
```

### Example

```bash
image_to_ascii_converter placeholders/chavito.png
```

The program will output the ASCII art to the console.

## How It Works

1. **Image Loading**: The application loads the image using the `image` crate
2. **Grayscale Conversion**: Converts the image to grayscale (luma8 format)
3. **Resizing**: Compresses the image to 200x200 pixels while maintaining aspect ratio
4. **ASCII Conversion**: Maps each pixel's brightness to one of 69 ASCII characters
5. **Output**: Prints the ASCII art to the console

## Character Set

The application uses a 69-character gradient ranging from darkest to brightest:

` ' . \` ^ " , : ; I l ! i > < ~ + _ - ? ] [ } { 1 ) ( | \\ / t f j r x n u v c z X Y U J C L Q 0 O Z m w q p d b k h a o * # M W & 8 % B @ $`

## Dependencies

- `image = "0.25.10"`: For image loading and processing

## License

This project is open source. See the LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request