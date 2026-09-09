# Desktop Release Packaging

Desktop release artifacts are built by `.github/workflows/release.yml` for tags
matching `v*`.

## Required GitHub Secrets

Tauri updater signing:

- `TAURI_SIGNING_PUBLIC_KEY`
- `TAURI_SIGNING_PRIVATE_KEY`
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` optional

macOS signing and notarization:

- `MACOS_CERTIFICATE_BASE64`
- `MACOS_CERTIFICATE_PASSWORD`
- `MACOS_SIGNING_IDENTITY`
- `APPLE_ID`
- `APPLE_PASSWORD`
- `APPLE_TEAM_ID`
- `APPLE_KEYCHAIN_PASSWORD`

## Desktop Assets

The desktop matrix publishes:

- `OmniSolo-vX.Y.Z-Windows.msi`
- `OmniSolo-vX.Y.Z-Windows.msi.sig`
- `OmniSolo-vX.Y.Z-Windows-Portable.zip`
- `OmniSolo-vX.Y.Z-macOS.dmg`
- `OmniSolo-vX.Y.Z-macOS.zip`
- `OmniSolo-vX.Y.Z-macOS.tar.gz`
- `OmniSolo-vX.Y.Z-macOS.tar.gz.sig`
- `OmniSolo-vX.Y.Z-Linux-x86_64.AppImage`
- `OmniSolo-vX.Y.Z-Linux-x86_64.AppImage.sig`
- `OmniSolo-vX.Y.Z-Linux-x86_64.deb`
- `OmniSolo-vX.Y.Z-Linux-x86_64.rpm`
- `OmniSolo-vX.Y.Z-Linux-arm64.AppImage`
- `OmniSolo-vX.Y.Z-Linux-arm64.AppImage.sig`
- `OmniSolo-vX.Y.Z-Linux-arm64.deb`
- `OmniSolo-vX.Y.Z-Linux-arm64.rpm`

After the GitHub Release is created, the workflow uploads `latest.json` for the
Tauri updater. macOS uses the signed `.tar.gz`, Windows uses the `.msi`, and
Linux uses the AppImage artifacts.
