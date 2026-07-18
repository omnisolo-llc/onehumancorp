#!/usr/bin/env bash
set -euo pipefail

rules_file="$1"
e2e_build_file="$2"
workflow_file="$3"

grep -Fq 'spec_args = [] if use_runfile_discovery else ci_specs' "$rules_file" || {
  echo "Playwright CI shards must each receive the complete curated spec set." >&2
  exit 1
}
grep -Fq 'extra_env = {"PLAYWRIGHT_SHARD": "{}/{}".format(index + 1, ci_shard_count)}' "$rules_file" || {
  echo "Playwright CI shards must delegate test-level partitioning to Playwright." >&2
  exit 1
}
grep -Fq 'ci_shard_count = 4' "$e2e_build_file" || {
  echo "The real-stack browser sweep must be split into four bounded-memory shards." >&2
  exit 1
}

for shard in 1 2 3 4; do
  target="//src/e2e:playwright_shard_${shard}_of_4"
  count="$(grep -Fxc "            target: ${target}" "$workflow_file")"
  if [[ "$count" != "1" ]]; then
    echo "Expected exactly one GitHub Actions entry for ${target}; found ${count}." >&2
    exit 1
  fi
done

if grep -Fq 'target: //src/e2e:playwright_shard_1_of_1' "$workflow_file"; then
  echo "The obsolete single-process Playwright shard is still wired into CI." >&2
  exit 1
fi
