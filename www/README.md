# Rust Music Theory Web Playground

An interactive web-based playground for exploring music theory concepts using the rust-music-theory library compiled to WebAssembly.

## Features

### Scales
- Generate scales with various types (Diatonic, Pentatonic, Blues, etc.)
- Support for all 7 modes (Ionian, Dorian, Phrygian, etc.)
- Configurable tonic, octave, and direction
- Real-time note display

### Chords
- Generate chords with different qualities (Major, Minor, Diminished, etc.)
- Support for extensions (Triads, 7th, 9th, 11th, 13th)
- Interactive chord explorer

## Usage

1. **Start the web server:**
   ```bash
   cd www
   python3 -m http.server 8000
   ```

2. **Open your browser:**
   Navigate to `http://localhost:8000`

3. **Explore music theory:**
   - Use the Scales tab to generate different scales
   - Use the Chords tab to explore chord structures
   - Switch between tabs using Ctrl/Cmd + 1/2
   - Generate with Ctrl/Cmd + Enter

## Building from Source

To rebuild the WASM module:

```bash
# Install wasm-pack if not already installed
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build the WASM module
wasm-pack build --target web --out-dir www/pkg
```

## Technical Details

- **Frontend:** Vanilla HTML, CSS, JavaScript
- **Backend:** Rust compiled to WebAssembly
- **Build Tool:** wasm-pack
- **WASM Bindings:** wasm-bindgen

## Keyboard Shortcuts

- `Ctrl/Cmd + 1`: Switch to Scales tab
- `Ctrl/Cmd + 2`: Switch to Chords tab
- `Ctrl/Cmd + Enter`: Generate scale/chord for active tab

## Browser Compatibility

Modern browsers that support:
- WebAssembly
- ES6 Modules
- CSS Grid

Tested on:
- Chrome 90+
- Firefox 90+
- Safari 14+
- Edge 90+