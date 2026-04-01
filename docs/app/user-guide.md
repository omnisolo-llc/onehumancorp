# User Guide: OHC Flutter App

## 1. Overview

This guide covers the Bazel-native Flutter app workflow in `srcs/app` for the One Human Corp platform.
The app's screenshots are generated from the local Flutter web bundle running on the Hybrid Agentic OS,
showcasing the Glassmorphism UI aesthetic, Outfit/Inter typography, and native integration features.

## 2. Regenerate Screenshots

Run either of the following from the repository root:

```bash
bazelisk run //srcs/app:capture_screenshots
```

Or use the VS Code task `App: Capture Flutter screenshots`.

Generated images are written to:

- `docs/app/web/`
- `docs/app/macos/`
- `docs/app/ios/`
- `docs/app/windows/`
- `docs/app/android/`
- `docs/app/linux/`

## 3. Screenshot Gallery

### Web

![OHC Flutter app on web](./web/login.png)

### macOS

![OHC Flutter app with macOS profile (Glassmorphism UI)](./macos/login.png)

### iOS

![OHC Flutter app with iOS profile (Glassmorphism UI)](./ios/login.png)

### Windows

![OHC Flutter app with Windows profile (Glassmorphism UI)](./windows/login.png)

### Android

![OHC Flutter app with Android profile (Glassmorphism UI)](./android/login.png)

### Linux

![OHC Flutter app with Linux profile (Glassmorphism UI)](./linux/login.png)
