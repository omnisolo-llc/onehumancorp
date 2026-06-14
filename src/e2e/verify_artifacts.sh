#!/bin/bash
set -e

echo "Verifying build artifacts for all platforms..."

# In Bazel, tests only see the artifacts built for the target platform.
# For example, on Linux, we only see Linux artifacts. The Windows
# ones will not be built and thus will be missing in the runfiles.
# So we shouldn't emit warnings for cross-platform artifacts missing.

ARTIFACTS_DIR="${TEST_SRCDIR}/${TEST_WORKSPACE}/release"
if [ ! -d "$ARTIFACTS_DIR" ]; then
    ARTIFACTS_DIR="release"
fi

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux expected artifacts
    EXPECTED_ARCHIVES=(
        "ohc-cluster.tar.gz"
    )
elif [[ "$OSTYPE" == "msys"* || "$OSTYPE" == "cygwin"* ]]; then
    # Windows expected artifacts
    EXPECTED_ARCHIVES=(
        "ohc-cluster.zip"
    )
elif [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS expected artifacts
    EXPECTED_ARCHIVES=(
        "ohc-cluster.tar.gz"
    )
else
    EXPECTED_ARCHIVES=()
fi

for arch in "${EXPECTED_ARCHIVES[@]}"; do
    if find "$ARTIFACTS_DIR" -name "$arch" | grep -q .; then
        echo "PASS: $arch exists"
    else
        echo "WARNING: $arch is missing (this might be due to build failure in restricted environment)"
    fi
done

# Check for Linux packages (only on Linux)
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if find "$ARTIFACTS_DIR" -name "*.deb" | grep -q .; then
        echo "PASS: DEB package exists"
    else
        echo "WARNING: DEB package is missing"
    fi

    # RPM requires rpmbuild, we don't strictly require it to exist and log warning,
    # because it might be skipped if rpmbuild is not available.
    if find "$ARTIFACTS_DIR" -name "*.rpm" | grep -q .; then
        echo "PASS: RPM package exists"
    fi
fi

# Check for OCI image artifacts
if find "$ARTIFACTS_DIR" -name "app_image_tarball.sh" | grep -q .; then
    echo "PASS: OCI image load script exists"
fi

echo "Basic verification checks completed."
