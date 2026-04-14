const { execFileSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const protocPath = process.argv[2];
const args = process.argv.slice(3);

const tsProtoBin = require.resolve('ts-proto/protoc-gen-ts_proto');

// Let protoc know where our plugin is
args.push(`--plugin=protoc-gen-ts_proto=${tsProtoBin}`);

let actualProtoc = protocPath;
const levels = (process.env.BAZEL_BINDIR || '').split('/').length;
let up = Array(levels).fill('..').join('/');

if (process.cwd().endsWith(process.env.BAZEL_BINDIR || 'unknown')) {
  // If we're inside BAZEL_BINDIR, we're likely in the wrong cwd for resolving files.
  // We need to change to the execroot.
  process.chdir(up);
}

actualProtoc = path.resolve(process.cwd(), protocPath);

try {
  execFileSync(actualProtoc, args, { stdio: 'inherit' });
} catch (error) {
  process.exit(error.status || 1);
}
