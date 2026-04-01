<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

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

- `docs/app/web/`
- `docs/app/macos/`
- `docs/app/ios/`
- `docs/app/windows/`
- `docs/app/android/`
- `docs/app/linux/`

## 3. Screenshot Gallery

### Web

![OHC Flutter app on web](../../srcs/app/web/login.png)

### macOS

![OHC Flutter app with macOS profile](../../srcs/app/macos/login.png)

### iOS

![OHC Flutter app with iOS profile](../../srcs/app/ios/login.png)

### Windows

![OHC Flutter app with Windows profile](../../srcs/app/windows/login.png)

### Android

![OHC Flutter app with Android profile](../../srcs/app/android/login.png)

### Linux

![OHC Flutter app with Linux profile](../../srcs/app/linux/login.png)


</div>
