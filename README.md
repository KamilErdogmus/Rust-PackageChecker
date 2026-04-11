# 🔧 Rust Package Checker

A cross-platform desktop application for scanning, monitoring, and updating packages across all major package managers — built with **Tauri 2**, **Rust**, and **React**.

![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-blue)
![Rust](https://img.shields.io/badge/rust-1.94%2B-orange)
![Tauri](https://img.shields.io/badge/tauri-2.x-purple)
![React](https://img.shields.io/badge/react-19.x-61DAFB)
![TypeScript](https://img.shields.io/badge/typescript-6.x-blue)
![License](https://img.shields.io/badge/license-MIT-green)

---

## 📋 Overview

Rust Package Checker gives you a unified dashboard to see all installed packages across every package manager on your system. It detects available updates, lets you apply or roll back updates in a single click, and keeps a full history of every change made.

---

## ✨ Features

- **Multi-Package Manager Support** — Winget, Chocolatey, Scoop, Homebrew, apt, dnf, pacman, npm, pip, gem, flatpak, snap
- **Cross-Platform** — Works natively on Windows, macOS, and Linux
- **Live Update Detection** — Scans all installed packages and reports available upgrades in real time
- **One-Click Updates** — Apply individual or batch updates from the dashboard
- **Rollback Support** — Revert packages to a previous version (Winget, npm, Chocolatey)
- **Package Details** — View description, publisher, version history, and category per package
- **Update History** — Full audit log of every update and rollback applied
- **Lightweight Binary** — Rust + Tauri means a tiny installer with near-zero runtime overhead

---

## 🚀 Advantages

### Why Rust + Tauri?

- **Memory safety** — Rust eliminates entire classes of bugs (null pointers, data races) at compile time
- **Tiny footprint** — Tauri apps ship as small native binaries, not bundled Electron runtimes
- **Blazing fast** — System calls and process management run at native speed with no JS overhead
- **Secure by default** — Tauri's capability model restricts what the frontend can access

### Why a unified package manager?

- Stop juggling multiple terminal windows and tools — everything is in one place
- Catch outdated packages you forgot about (that npm global from 2022, etc.)
- Safe rollback means you can update confidently without fear of breaking your environment

---

## 🛠️ Tech Stack

| Layer          | Technology                             |
| -------------- | -------------------------------------- |
| Frontend       | React 19, TypeScript 6, Tailwind CSS 4 |
| Backend        | Rust (Tauri 2 commands)                |
| State          | Zustand 5                              |
| Styling        | shadcn/ui + Framer Motion 12           |
| Async runtime  | Tokio                                  |
| Serialization  | serde / serde_json                     |
| Error handling | thiserror                              |
| Logging        | tracing / tracing-subscriber           |
| Build tool     | Vite 8                                 |

---

## 📦 Supported Package Managers

| Platform           | Package Managers                     |
| ------------------ | ------------------------------------ |
| **Windows**        | Winget, Chocolatey, Scoop            |
| **macOS**          | Homebrew                             |
| **Linux**          | apt, dnf, pacman, Flatpak, Snap      |
| **Cross-platform** | npm (global), pip / pip3, gem (Ruby) |

---

## 🔧 Prerequisites

- [Rust](https://rustup.rs/) 1.94+
- [Bun](https://bun.sh/) or [npm](https://www.npmjs.com/) (comes with Node.js)
- [Tauri prerequisites](https://tauri.app/start/prerequisites/) for your platform

---

## 🚀 Getting Started

```bash
# Clone the repository
git clone https://github.com/KamilErdogmus/Rust-PackageChecker.git
cd Rust-PackageChecker
```

**Using Bun (recommended):**

```bash
bun install
bun run tauri
bun run tauri:build
```

**Using npm:**

```bash
npm install
npm run tauri
npm run tauri:build
```

---

## 📖 How It Works

1. **Scan** — The app queries every detected package manager and aggregates all installed packages
2. **Detect** — It runs each package manager's "outdated / upgrade check" command and collects available updates
3. **Apply** — Updates are dispatched to the correct adapter which runs the appropriate install command
4. **History** — Every action is logged so you can audit or roll back later

---

## 🤝 Contributing

Contributions are welcome! Please open an issue first to discuss what you'd like to change.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📄 License

This project is licensed under the MIT License.
