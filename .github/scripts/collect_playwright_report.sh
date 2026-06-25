#!/usr/bin/env bash
set -euo pipefail

target_name="${BAZEL_TARGET##*:}"
testlog_dir="bazel-testlogs/src/e2e/${target_name}"
report_dir="$RUNNER_TEMP/playwright-ci-artifacts/${ARTIFACT_NAME}"
artifact_url="https://github.com/${GITHUB_REPOSITORY}/actions/runs/${GITHUB_RUN_ID}#artifacts"
retention_days="${PLAYWRIGHT_REPORT_RETENTION_DAYS:-30}"
matrix_name="${MATRIX_NAME:-$target_name}"

mkdir -p "$report_dir"

{
  echo "# Playwright UI E2E report"
  echo
  echo "- Matrix job: $matrix_name"
  echo "- Bazel target: \`$BAZEL_TARGET\`"
  echo "- Artifact: \`$ARTIFACT_NAME\`"
  echo "- Retention: $retention_days days"
  echo "- Run artifacts: $artifact_url"
  echo
  echo "If present, open \`undeclared-outputs/report/index.html\` after downloading the artifact to inspect the Playwright HTML report."
} > "$report_dir/README.md"

if [[ -f "$testlog_dir/test.log" ]]; then
  cp "$testlog_dir/test.log" "$report_dir/bazel-test.log"
fi

if [[ -d "$testlog_dir/test.outputs" ]]; then
  mkdir -p "$report_dir/bazel-test-outputs"
  cp -R "$testlog_dir/test.outputs/." "$report_dir/bazel-test-outputs/"
fi

if [[ -f "$testlog_dir/test.outputs/outputs.zip" ]]; then
  mkdir -p "$report_dir/undeclared-outputs"
  python3 - "$testlog_dir/test.outputs/outputs.zip" "$report_dir/undeclared-outputs" <<'PY'
import sys
import zipfile

zip_path = sys.argv[1]
output_dir = sys.argv[2]
with zipfile.ZipFile(zip_path) as archive:
    archive.extractall(output_dir)
PY
fi

find "$report_dir" -maxdepth 4 -type f | sort > "$report_dir/files.txt"

{
  echo "### Playwright UI E2E report: $matrix_name"
  echo
  echo "- Artifact: \`$ARTIFACT_NAME\`"
  echo "- Retention: $retention_days days"
  echo "- Run artifacts: $artifact_url"
  echo "- HTML report path after download, if present: \`undeclared-outputs/report/index.html\`"
  echo
  echo "<details><summary>Collected report files</summary>"
  echo
  sed 's#^#- `#; s#$#`#' "$report_dir/files.txt"
  echo
  echo "</details>"
} >> "$GITHUB_STEP_SUMMARY"
