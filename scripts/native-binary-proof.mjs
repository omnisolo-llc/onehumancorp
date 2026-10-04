// Same-build producer receipt for native E2E reuse. A source snapshot belongs
// before the compiler command; record belongs only after that command succeeds.
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { createReadStream } from 'node:fs';
import { lstat, mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const fail = () => { throw new Error('Native binary proof does not match the current source and compiler outputs; rebuild and record it after success'); };
export async function nativeSourceDigest(repository = root, targetDirectory = path.resolve(repository, process.env.CARGO_TARGET_DIR ?? 'target')) {
  // Include new untracked source too. Dependency/build directories and local
  // secrets are never compiler source inputs or accepted as source symlinks.
  const output = execFileSync('git', ['ls-files', '--cached', '--others', '--exclude-standard', '-z'], {
    cwd: repository, encoding: 'utf8', maxBuffer: 16 * 1024 * 1024,
  });
  const targetRelative = path.relative(repository, targetDirectory).split(path.sep).join('/');
  if (!targetRelative || targetRelative === '.') fail();
  const files = [...new Set(output.split('\0').filter(Boolean))].filter(name => name !== targetRelative && !name.startsWith(`${targetRelative}/`)
    && !name.split('/').some(part =>
    ['node_modules', 'target', '.next', '.git', 'coverage', 'test-results', 'playwright-report', 'next-env.d.ts'].includes(part)
    || part.endsWith('.tsbuildinfo') || /^\.env(?:\.|$)/.test(part))).sort();
  if (!files.length) fail();
  const hash = createHash('sha256');
  for (const name of files) {
    const filename = path.join(repository, name), stat = await lstat(filename);
    if (!stat.isFile() || stat.isSymbolicLink()) fail();
    hash.update(JSON.stringify([name, stat.size]));
    for await (const chunk of createReadStream(filename)) hash.update(chunk);
  }
  return hash.digest('hex');
}
async function fileIdentity(filename) {
  const stat = await lstat(filename);
  if (!stat.isFile() || stat.isSymbolicLink() || stat.size === 0) fail();
  const hash = createHash('sha256');
  for await (const chunk of createReadStream(filename)) hash.update(chunk);
  return { bytes: stat.size, sha256: hash.digest('hex') };
}
async function readProof(filename) {
  const stat = await lstat(filename);
  if (!stat.isFile() || stat.isSymbolicLink() || stat.size > 8192) fail();
  return JSON.parse(await readFile(filename, 'utf8'));
}
export async function snapshotNativeSource(filename, repository = root, targetDirectory) {
  const value = { schemaVersion: 1, sourceSha256: await nativeSourceDigest(repository, targetDirectory) };
  await mkdir(path.dirname(path.resolve(filename)), { recursive: true });
  await writeFile(filename, JSON.stringify(value) + '\n', { mode: 0o600 });
  return value;
}
export async function recordNativeBinaryProof(snapshotPath, proofPath, binaries, repository = root) {
  const snapshot = await readProof(snapshotPath);
  const targetDirectory = path.dirname(path.dirname(binaries.server));
  if (snapshot.schemaVersion !== 1 || snapshot.sourceSha256 !== await nativeSourceDigest(repository, targetDirectory)) fail();
  const value = { schemaVersion: 1, sourceSha256: snapshot.sourceSha256,
    platform: process.platform, architecture: process.arch,
    server: await fileIdentity(binaries.server), agent: await fileIdentity(binaries.agent) };
  if (snapshot.sourceSha256 !== await nativeSourceDigest(repository, targetDirectory)) fail();
  await writeFile(proofPath, JSON.stringify(value) + '\n', { mode: 0o600 });
  return value;
}
export async function verifyNativeBinaryProof(proofPath, binaries, repository = root) {
  const proof = await readProof(proofPath);
  const targetDirectory = path.dirname(path.dirname(binaries.server));
  if (proof.schemaVersion !== 1 || proof.platform !== process.platform || proof.architecture !== process.arch
      || proof.sourceSha256 !== await nativeSourceDigest(repository, targetDirectory)) fail();
  for (const name of ['server', 'agent']) {
    const observed = await fileIdentity(binaries[name]);
    if (proof[name]?.sha256 !== observed.sha256 || proof[name]?.bytes !== observed.bytes) fail();
  }
  if (proof.sourceSha256 !== await nativeSourceDigest(repository, targetDirectory)) fail();
  return proof;
}
if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) {
  const [action, first, second] = process.argv.slice(2);
  const directory = path.resolve(root, process.env.CARGO_TARGET_DIR ?? 'target', 'debug');
  const suffix = process.platform === 'win32' ? '.exe' : '';
  const binaries = { server: path.join(directory, `server${suffix}`), agent: path.join(directory, `omnisolo-builtin-agent${suffix}`) };
  const run = async () => {
    if (action === 'snapshot' && first && !second) return snapshotNativeSource(first);
    if (action === 'record' && first && second) return recordNativeBinaryProof(first, second, binaries);
    if (action === 'verify' && first && !second) return verifyNativeBinaryProof(first, binaries);
    throw new Error('Expected snapshot <snapshot.json>, record <snapshot.json> <proof.json>, or verify <proof.json>');
  };
  run().catch(error => { console.error(error.message); process.exitCode = 1; });
}
