# osu!Reffer

A modern, cross-platform IRC client specifically designed for osu! referees and tournament organizers. Built with Tauri, Vue 3, and TypeScript, osu!Reffer provides an intuitive interface for managing osu! multiplayer lobbies and communicating through Bancho IRC.

## 🚀 Getting Started

### For Users

#### Download & Installation
1. Visit the [Releases](https://github.com/V1laZ/easyOsuReffer/releases) page
2. Download the appropriate version for your operating system
3. Install and run the application

### For Developers

#### Prerequisites
- [Node.js](https://nodejs.org/) (v18 or higher)
- [pnpm](https://pnpm.io/) package manager
- [Rust](https://rustup.rs/) (latest stable)

#### Installation
```bash
# Clone the repository
git clone https://github.com/V1laZ/easyOsuReffer.git
cd easyOsuReffer

# Install dependencies
pnpm install
```

#### Development

##### Start Development Server
```bash
# Start the development server
pnpm tauri dev
```

##### Build for Production
```bash
# Build the application
pnpm tauri build
```

##### Mobile Development (Android)
```bash
# Add Android target
pnpm tauri android init

# Run on physical Android device
pnpm tauri android dev

# Run on Android emulator
pnpm tauri android dev --open

# Build Android APK
pnpm tauri android build
```
---

Made with ❤️ for the osu! community