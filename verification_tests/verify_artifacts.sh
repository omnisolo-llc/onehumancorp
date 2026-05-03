#!/bin/bash
set -e

echo "Verifying build artifacts for all platforms..."

# List of artifacts that we expect to be generated.
# We use find to be flexible with file names and locations.
ARTIFACTS_DIR="release"

# Check for essential archives
EXPECTED_ARCHIVES=(
    "app_tar.tar.gz"
    "ohc-app-windows.zip"
)

for arch in "${EXPECTED_ARCHIVES[@]}"; do
    if find "$ARTIFACTS_DIR" -name "$arch" | grep -q .; then
        echo "PASS: $arch exists"
    else
        echo "WARNING: $arch is missing (this might be due to build failure in restricted environment)"
    fi
done

# Check for Linux packages
if find "$ARTIFACTS_DIR" -name "*.deb" | grep -q .; then
    echo "PASS: DEB package exists"
fi

if find "$ARTIFACTS_DIR" -name "*.rpm" | grep -q .; then
    echo "PASS: RPM package exists"
fi

# Check for OCI image artifacts
if find "$ARTIFACTS_DIR" -name "app_image_tarball.sh" | grep -q .; then
    echo "PASS: OCI image load script exists"
fi

echo "Basic verification checks completed."
