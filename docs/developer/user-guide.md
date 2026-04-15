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

- `docs/public/assets/screenshots/app/landing-page/`

> **Note:** The screenshot gallery below requires running the capture target first.
> Images are not committed to source control.

## 3. Screenshot Gallery

Screenshots are captured by running `bazelisk run //srcs/app:capture_screenshots`.
The gallery will populate with images at:
- `docs/public/assets/screenshots/app/landing-page/web.png`
- `docs/public/assets/screenshots/app/landing-page/macos.png`
- `docs/public/assets/screenshots/app/landing-page/windows.png`
- `docs/public/assets/screenshots/app/landing-page/android.png`
- `docs/public/assets/screenshots/app/landing-page/ios.png`
- `docs/public/assets/screenshots/app/landing-page/linux.png`

</div>
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

## 4. Documentation

Please refer to the detailed architecture documents in the `docs/` folder:
- [KAIROS Orchestration Design Phase 4](../architecture/kairos/orchestration-phase4.md)

</div>
