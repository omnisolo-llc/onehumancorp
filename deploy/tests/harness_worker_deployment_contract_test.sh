#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

reject_file_pattern() {
  local pattern="$1"
  local file="$2"
  if grep -Eq "$pattern" "$file"; then
    echo "forbidden pattern '$pattern' found in $file" >&2
    return 1
  fi
}

grep -q '^harnessWorkers:' "$repo_root/deploy/helm/ohc/values.yaml"
grep -q '^modelRuntimeWorkers:' "$repo_root/deploy/helm/ohc/values.yaml"
grep -q 'kind: Deployment' "$repo_root/deploy/helm/ohc/templates/harness-workers.yaml"
grep -q 'kind: Service' "$repo_root/deploy/helm/ohc/templates/harness-workers.yaml"
grep -q 'kind: HorizontalPodAutoscaler' "$repo_root/deploy/helm/ohc/templates/harness-workers-hpa.yaml"
grep -q 'kind: Deployment' "$repo_root/deploy/helm/ohc/templates/model-runtime-workers.yaml"
grep -q 'kind: Service' "$repo_root/deploy/helm/ohc/templates/model-runtime-workers.yaml"
grep -q 'kind: HorizontalPodAutoscaler' "$repo_root/deploy/helm/ohc/templates/model-runtime-workers-hpa.yaml"
grep -q 'path: /healthz' "$repo_root/deploy/helm/ohc/templates/harness-workers.yaml"
grep -q 'path: /readyz' "$repo_root/deploy/helm/ohc/templates/harness-workers.yaml"
grep -q 'terminationGracePeriodSeconds:' "$repo_root/deploy/helm/ohc/templates/harness-workers.yaml"
grep -q 'terminationGracePeriodSeconds: 120' "$repo_root/deploy/helm/ohc/values.yaml"
grep -q 'secretKeyRef:' "$repo_root/deploy/helm/ohc/templates/harness-workers.yaml"
grep -q 'emptyDir:' "$repo_root/deploy/helm/ohc/templates/harness-workers.yaml"
grep -q 'OMNISOLO_MODEL_RUNTIME_RUNTIME_ID' "$repo_root/deploy/helm/ohc/templates/model-runtime-workers.yaml"

harnesses=(omnisolo codex opencode deepseek pi kimi openhands openharness aider goose open-interpreter plandex)
for harness in "${harnesses[@]}"; do
  grep -q "harness-$harness:" "$repo_root/deploy/docker-compose.yml"
  grep -q -- "- id: $harness" "$repo_root/deploy/helm/ohc/values.yaml"
done

for harness in aider goose open-interpreter plandex; do
  grep -q "OMNISOLO_HARNESS_EXECUTABLE: omnisolo-openai-shim" "$repo_root/deploy/docker-compose.yml"
  grep -q "OMNISOLO_HARNESS_PROTOCOL: openai_compatible_shim" "$repo_root/deploy/docker-compose.yml"
  grep -q "OMNISOLO_HARNESS_EXECUTABLE: omnisolo-openai-shim" "$repo_root/deploy/helm/ohc/values.yaml"
  grep -q "OMNISOLO_HARNESS_PROTOCOL: openai_compatible_shim" "$repo_root/deploy/helm/ohc/values.yaml"
done
grep -q 'COPY --chmod=0755 src/server/harness/sidecars/openai_compatible_shim.py /usr/local/bin/omnisolo-openai-shim' "$repo_root/deploy/docker/Dockerfile.harness-worker"

worker_images=(
  'omnisolo/harness-worker:0.1.0'
  'omnisolo/harness-worker-codex:0.149.0'
  'omnisolo/harness-worker-opencode:1.18.15'
  'omnisolo/harness-worker-deepseek:0.1.1-rc.2'
  'omnisolo/harness-worker-pi:0.73.1'
  'omnisolo/harness-worker-kimi:1.49.0'
  'omnisolo/harness-worker-openhands:1.43.1'
  'omnisolo/harness-worker-openharness:0.6.0'
  'omnisolo/harness-worker-aider:0.86.0'
  'omnisolo/harness-worker-goose:1.33.1'
  'omnisolo/harness-worker-open-interpreter:0.4.2'
  'omnisolo/harness-worker-plandex:2.2.1'
)
for image in "${worker_images[@]}"; do
  grep -q "$image" "$repo_root/deploy/docker-compose.yml"
  grep -q "$image" "$repo_root/deploy/helm/ohc/values.yaml"
done
reject_file_pattern 'omnisolo/harness-worker[^ }]*:latest' "$repo_root/deploy/docker-compose.yml"
reject_file_pattern 'omnisolo/harness-worker[^ ]*:latest' "$repo_root/deploy/helm/ohc/values.yaml"

test "$(grep -c '^  harness-[a-z].*:$' "$repo_root/deploy/docker-compose.yml")" -eq 12
test "$(grep -c 'stop_grace_period:.*OMNISOLO_HARNESS_STOP_GRACE_PERIOD' "$repo_root/deploy/docker-compose.yml")" -eq 12
test "$(grep -c '^    - id: \(omnisolo\|codex\|opencode\|deepseek\|pi\|kimi\|openhands\|openharness\|aider\|goose\|open-interpreter\|plandex\)$' "$repo_root/deploy/helm/ohc/values.yaml")" -eq 12
grep -q 'COPY --chmod=0755 src/server/harness/sidecars/kimi_acp_bridge.py /usr/local/bin/omnisolo-kimi-acp' \
  "$repo_root/deploy/docker/Dockerfile.harness-worker"
grep -q 'OMNISOLO_HARNESS_EXECUTABLE: omnisolo-kimi-acp' "$repo_root/deploy/docker-compose.yml"
grep -q 'OMNISOLO_HARNESS_EXECUTABLE: omnisolo-kimi-acp' "$repo_root/deploy/helm/ohc/values.yaml"
grep -q 'executables=.*omnisolo-kimi-acp' "$repo_root/scripts/test-live-harness-matrix.sh"
grep -q 'OMNISOLO_HARNESS_EXECUTABLE: openhands-agent-server' "$repo_root/deploy/docker-compose.yml"
grep -q 'OMNISOLO_HARNESS_EXECUTABLE: openhands-agent-server' "$repo_root/deploy/helm/ohc/values.yaml"
grep -q 'executables=.*openhands-agent-server' "$repo_root/scripts/test-live-harness-matrix.sh"
awk '/^  harness-openhands:/{found=1} found && /- \/tmp:/{print; exit}' \
  "$repo_root/deploy/docker-compose.yml" | grep -q '/tmp:exec,mode=1777'
grep -q 'tmp_mount="/tmp:exec,mode=1777"' "$repo_root/scripts/test-live-harness-matrix.sh"
grep -q '{{.State.Running}}' "$repo_root/scripts/test-live-harness-matrix.sh"
grep -q 'OMNISOLO_HARNESS_REQUEST_TIMEOUT_SECS=600' "$repo_root/scripts/test-live-harness-matrix.sh"
grep -q 'ghcr.io/openhands/agent-server:1.43.1-python@sha256:6f5c614cdab68150d6365e5be1051de4a005375dc46249e17ae2349ec93a9cc0 AS openhands' \
  "$repo_root/deploy/docker/Dockerfile.harness-worker"
grep -q 'command -v openhands-agent-server' "$repo_root/deploy/docker/Dockerfile.harness-worker"
grep -q 'openhands-agent-server --help' "$repo_root/deploy/docker/Dockerfile.harness-worker"
test "$(grep -c '/workspace:mode=0700,uid=1000,gid=1000' "$repo_root/deploy/docker-compose.yml")" -eq 12
test "$(grep -c '/home/omnisolo:mode=0700,uid=1000,gid=1000' "$repo_root/deploy/docker-compose.yml")" -eq 12
grep -q 'mountPath: /workspace' "$repo_root/deploy/helm/ohc/templates/harness-workers.yaml"
grep -q 'mountPath: /home/omnisolo' "$repo_root/deploy/helm/ohc/templates/harness-workers.yaml"
grep -q '^WORKDIR /workspace$' "$repo_root/deploy/docker/Dockerfile.harness-worker"

# The native OpenCode adapter owns `serve` and loopback-port arguments. Worker
# configuration must provide only executable-prefix arguments or it will launch
# `opencode serve ... serve ...`.
awk '/^  harness-opencode:/{found=1} found && /OMNISOLO_HARNESS_ARGS_JSON:/{print; exit}' \
  "$repo_root/deploy/docker-compose.yml" | grep -q '\[\]'
awk '/^    - id: opencode$/{found=1} found && /OMNISOLO_HARNESS_ARGS_JSON:/{print; exit}' \
  "$repo_root/deploy/helm/ohc/values.yaml" | grep -q "'\[\]'"

for file in "$repo_root/deploy/docker-compose.yml" "$repo_root/deploy/helm/ohc/values.yaml"; do
  grep -q 'OPENAI_API_KEY' "$file"
  grep -q 'OPENAI_API_BASE_URL' "$file"
  grep -q 'OPENAI_MODEL' "$file"
  grep -q 'OPENAI_REASONING_EFFORT' "$file"
  grep -q 'gpt-5.6-luna' "$file"
  grep -q 'max' "$file"
  reject_file_pattern 'OMNISOLO_HARNESS_API_KEY' "$file"
  reject_file_pattern 'OMNISOLO_HARNESS_BASE_URL' "$file"
  reject_file_pattern 'CODEX_HOME' "$file"
done

for marker in \
  '@openai/codex@0.149.0' \
  'opencode-ai@1.18.15' \
  '@deepseek-ai/dsh-sdk-jsonrpc-demo@0.1.1-rc.2' \
  '@deepseek-ai/dsh-terminal@0.1.1-rc.2' \
  '@deepseek-ai/dsh-terminal-bash@0.1.1-rc.2' \
  '@deepseek-ai/dsh-fs-local@0.1.1-rc.2' \
  '@deepseek-ai/dsh-tool-bash-persistent@0.1.1-rc.2' \
  '@deepseek-ai/dsh-tool-str-replace-editor@0.1.1-rc.2' \
  '@mariozechner/pi-coding-agent@0.73.1' \
  'kimi-cli==1.49.0' \
  'harness-agent==0.6.0'; do
  grep -Rq "$marker" \
    "$repo_root/deploy/docker/Dockerfile.harness-worker" \
    "$repo_root/deploy/harness"
done

for marker in \
  'aider-chat==0.86.0' \
  'v1.33.1' \
  'goose-x86_64-unknown-linux-gnu.tar.gz' \
  'open-interpreter==0.4.2' \
  'cli/v2.2.1' \
  'plandex_2.2.1_linux_amd64.tar.gz'; do
  grep -Rq "$marker" \
    "$repo_root/deploy/docker/Dockerfile.harness-worker" \
    "$repo_root/deploy/harness"
done

test "$(grep -c -- '--require-hashes' "$repo_root/deploy/docker/Dockerfile.harness-worker")" -eq 2
for harness in kimi openharness; do
  test -f "$repo_root/deploy/harness/$harness/requirements.in"
  test -f "$repo_root/deploy/harness/$harness/requirements.lock"
  grep -q -- '--hash=sha256:' "$repo_root/deploy/harness/$harness/requirements.lock"
  grep -q "COPY deploy/harness/$harness/requirements.lock" \
    "$repo_root/deploy/docker/Dockerfile.harness-worker"
done
test ! -e "$repo_root/deploy/harness/openhands/requirements.in"
test ! -e "$repo_root/deploy/harness/openhands/requirements.lock"

grep -q 'DSH_CORDIS_CONFIG' "$repo_root/deploy/helm/ohc/values.yaml"
grep -q "@deepseek-ai/dsh-llm-pi-ai" "$repo_root/deploy/harness/deepseek/cordis.yml"
grep -q 'apiKeyEnv: OPENAI_API_KEY' "$repo_root/deploy/harness/deepseek/cordis.yml"
grep -q "api: openai-responses" "$repo_root/deploy/harness/deepseek/cordis.yml"
grep -q 'maxBytes:' "$repo_root/deploy/harness/deepseek/cordis.yml"
reject_file_pattern 'toolBash: false' "$repo_root/deploy/harness/deepseek/cordis.yml"
reject_file_pattern 'skills:.*false|enabled: false' "$repo_root/deploy/harness/deepseek/cordis.yml"
grep -q 'COPY deploy/harness/deepseek/cordis.yml /etc/omnisolo/deepseek/cordis.yml' "$repo_root/deploy/docker/Dockerfile.harness-worker"
grep -q 'secretName:' "$repo_root/deploy/helm/ohc/values.yaml"
grep -q 'autoscaling:' "$repo_root/deploy/helm/ohc/values.yaml"
grep -q 'model-runtime-default:' "$repo_root/deploy/docker-compose.yml"
test -f "$repo_root/deploy/docker/Dockerfile.harness-worker"
test -f "$repo_root/.dockerignore"
grep -q '^target/$' "$repo_root/.dockerignore"
grep -q '^\.worktrees/$' "$repo_root/.dockerignore"
grep -q '^scratch/$' "$repo_root/.dockerignore"
grep -q '^\*\*/node_modules/$' "$repo_root/.dockerignore"
grep -q '^\*\*/target/$' "$repo_root/.dockerignore"
grep -q '^\*\*/\.next/$' "$repo_root/.dockerignore"
grep -q 'protobuf-compiler' "$repo_root/deploy/docker/Dockerfile.harness-worker"
grep -q 'apt-get install.*ca-certificates wget' "$repo_root/deploy/docker/Dockerfile.harness-worker"
awk '/^FROM node:/{found=1; next} found && /^COPY --from=build/{exit} found{print}' \
  "$repo_root/deploy/docker/Dockerfile.harness-worker" | grep -q 'usermod --login omnisolo.*node'

echo "harness worker deployment contract passed"
