const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const protocArg = process.argv[2];
let protocPath = protocArg;

// Sometimes the protoc path is relative, let's resolve it against the CWD
if (!fs.existsSync(protocPath)) {
    protocPath = path.resolve(process.cwd(), protocArg);
}

// In Bazel sandbox, the CWD is `bazel-out/k8-fastbuild/bin`, but the protoc executable
// passed might be `bazel-out/k8-opt-exec/bin/external/protobuf+/protoc`.
// So it would be resolved to `bazel-out/k8-fastbuild/bin/bazel-out/k8-opt-exec/bin/external/protobuf+/protoc`.
// Let's also check if it exists in the original workspace root.

if (!fs.existsSync(protocPath)) {
    // If the path starts with `bazel-out`, we can try going up two directories to reach the exec root
    if (process.cwd().includes('bazel-out')) {
        const execRootPath = path.resolve(process.cwd(), '../../..', protocArg);
        if (fs.existsSync(execRootPath)) {
            protocPath = execRootPath;
        }
    }
}

if (!fs.existsSync(protocPath)) {
    console.error(`Cannot find protoc at ${protocArg}`);
    process.exit(1);
}

const args = process.argv.slice(3);

let newArgs = args;
// The rule sets --proto_path=., but we are running in the bin dir.
// Let's change the --proto_path to the exec root so `srcs/proto/hub.proto` can be found.
if (process.cwd().includes('bazel-out')) {
    const execRoot = path.resolve(process.cwd(), '../../..');
    newArgs = args.map(arg => {
        if (arg === '--proto_path=.') {
            return '--proto_path=' + execRoot;
        }
        if (arg.startsWith('--ts_proto_out=')) {
           // Create the output directory if it doesn't exist. Sometimes bazel might expect the script to do it.
           const outDir = arg.split('=')[1];
           const resolvedOutDir = path.isAbsolute(outDir) ? outDir : path.resolve(execRoot, outDir);
           if (!fs.existsSync(resolvedOutDir)) {
               fs.mkdirSync(resolvedOutDir, { recursive: true });
           }
           return `--ts_proto_out=${resolvedOutDir}`;
        }
        return arg;
    });
    // We also need to map the proto files to be relative to the exec root or absolute
    newArgs = newArgs.map(arg => {
        if (arg.endsWith('.proto') && !arg.startsWith('--')) {
            return path.resolve(execRoot, arg);
        }
        return arg;
    });
}

// Ensure the ts-proto plugin script has the correct execute permissions or we can run it through node directly
const tsProtoPluginPath = require.resolve('ts-proto/protoc-gen-ts_proto');

// Find the node executable from process.execPath so we use the correct hermetic node
const nodePath = process.execPath;

// protoc gen plugins don't have to be executable if we pass the binary path to --plugin
// Note: windows needs .cmd but linux does not. However, we can trick protoc by creating a tiny script
// Or better yet, just point protoc to the bin
let pluginArg;
if (fs.existsSync(tsProtoPluginPath)) {
    // The problem is that protoc expects the plugin argument to be an executable or a path to one
    // But ts-proto/protoc-gen-ts_proto is a javascript file. It has a shebang `#!/usr/bin/env node`
    // which might not work hermetically.
    // Instead we can create a temporary shell script that uses the hermetic node to run it.
    const tempWrapper = path.join(process.cwd(), 'hermetic-ts-proto.sh');
    fs.writeFileSync(tempWrapper, `#!/bin/sh\n"${nodePath}" "${tsProtoPluginPath}" "$@"\n`);
    fs.chmodSync(tempWrapper, 0o755);
    pluginArg = `--plugin=protoc-gen-ts_proto=${tempWrapper}`;
} else {
    pluginArg = `--plugin=protoc-gen-ts_proto=${nodePath} ${tsProtoPluginPath}`;
}

const protocArgs = [
  ...newArgs,
  pluginArg
];

try {
  execSync(`"${protocPath}" ${protocArgs.join(' ')}`, {
    stdio: 'inherit',
    env: process.env
  });
} catch (e) {
  process.exit(1);
}
