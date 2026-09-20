#!/usr/bin/env python3
"""Attach CI-supplied signing material to Tauri's generated Android project."""
from pathlib import Path
import base64
import os

required = ['ANDROID_KEYSTORE_BASE64', 'ANDROID_KEYSTORE_PASSWORD', 'ANDROID_KEY_ALIAS', 'ANDROID_KEY_PASSWORD']
missing = [key for key in required if not os.environ.get(key)]
if missing:
    raise SystemExit('Release signing configuration missing: ' + ', '.join(missing))
key = Path(os.environ['RUNNER_TEMP']) / 'ohc-android-release.keystore'
key.write_bytes(base64.b64decode(os.environ['ANDROID_KEYSTORE_BASE64'], validate=True))
key.chmod(0o600)
with open(os.environ['GITHUB_ENV'], 'a', encoding='utf-8') as environment:
    environment.write(f'ANDROID_KEYSTORE_PATH={key}\n')
project = Path('src/ui/tauri/gen/android/app/build.gradle.kts')
source = project.read_text()
marker = '// OHC native CI signing'
if marker in source:
    raise SystemExit('Signing was already configured; refusing to duplicate generated settings')
if source.count('android {') != 1 or source.count('getByName("release") {') != 1:
    raise SystemExit('Generated Android project format changed; review the signing integration')
source = source.replace('android {', '''android {
    // OHC native CI signing
    signingConfigs {
        create("release") {
            storeFile = file(System.getenv("ANDROID_KEYSTORE_PATH"))
            storePassword = System.getenv("ANDROID_KEYSTORE_PASSWORD")
            keyAlias = System.getenv("ANDROID_KEY_ALIAS")
            keyPassword = System.getenv("ANDROID_KEY_PASSWORD")
        }
    }
''', 1)
source = source.replace('getByName("release") {', 'getByName("release") {\n            signingConfig = signingConfigs.getByName("release")', 1)
project.write_text(source)
print('Configured release signing without writing passwords to source or build artifacts.')
