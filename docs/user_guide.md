# User Guide: OHC Flutter App

## 1. Overview

This guide covers the Bazel-native Flutter app workflow in `srcs/app`.
The app's screenshots are generated from the Bazel-built Flutter web bundle by
running Playwright with platform-specific viewport and device profiles.

## 2. Regenerate Screenshots

Run either of the following from the repository root:

```bash
bazelisk run //srcs/app:capture_screenshots
```

Or use the VS Code task `App: Capture Flutter screenshots`.

Generated images are written to:

- `docs/public/assets/screenshots/app/landing-page/`

## 3. Screenshot Gallery

### Web

![OHC Flutter app on web](./app/web/landing.png)
![OHC Flutter app login on web](./app/web/login.png)

### macOS

![OHC Flutter app with macOS profile](./app/macos/landing.png)
![OHC Flutter app login with macOS profile](./app/macos/login.png)

### Windows

![OHC Flutter app with Windows profile](./app/windows/landing.png)
![OHC Flutter app login with Windows profile](./app/windows/login.png)

### Android

![OHC Flutter app with Android profile](./app/android/landing.png)
![OHC Flutter app login with Android profile](./app/android/login.png)

### iOS

![OHC Flutter app with iOS profile](./app/ios/landing.png)
![OHC Flutter app login with iOS profile](./app/ios/login.png)

### Linux

![OHC Flutter app with Linux profile](./app/linux/landing.png)
![OHC Flutter app login with Linux profile](./app/linux/login.png)
## 4. Documentation

Please refer to the detailed architecture documents in the `docs/` folder:
- [KAIROS Orchestration Design Phase 4](./kairos_orchestration_phase4.md)

Last synced: 2026-04-17 03:42:41


## 5. Visual Rendering Specs

The OHC visual styling mandate requires components to have premium glassmorphism styling as text-based implementation specs for downstream agents:
* `backdrop-filter: blur(20px) saturate(200%);`
* `background: rgba(255, 255, 255, 0.05);`
* `border: 1px solid rgba(255, 255, 255, 0.1);`
* `font-family: 'Outfit', 'Inter', sans-serif;`

### 5.1 Architecture

```mermaid
graph TD;
    CloudMode[Cloud-Native Mode] -->|Sync| SIP[(OHC-SIP Vector DB)];
    StandaloneMode[Standalone Mode] -->|Sync| SIP;
    ThinClientMode[Thin Client Mode] -->|API/OAuth| CloudMode;
```

### 5.2 Comparative Analysis

| Feature | Cloud Mode | Standalone Mode | Thin Client Mode |
| :--- | :--- | :--- | :--- |
| **Orchestration** | K8s | SQLite Local | N/A |
| **Resources** | High | Low | Very Low |
| **Multi-tenant** | Yes | No | Yes |
