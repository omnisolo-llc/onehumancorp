sed -i 's|"go.yaml.in/yaml/v2"|"gopkg.in/yaml.v2"|' go.mod go.sum MODULE.bazel pnpm-lock.yaml pubspec.lock pnpm-workspace.yaml || true
