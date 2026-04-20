#!/bin/bash
set -euo pipefail

# This script runs the dart domain model generator hermetically.
# It is called by the Bazel rule proto_dart_library.

DART="external/rules_flutter/flutter/dart"
if [ ! -x "$DART" ]; then
    # Compatibility with older path style or different platforms
    DART=$(find external -name dart | head -n 1)
fi

# The first argument is the input .pb.dart relative path
# The second argument is the output .domain.dart relative path

# We need to run from the root of the execution sandbox
"$DART" bazel/rules/dart/tools/model_gen/generate_domain_models.dart "$@"
