// Desktop distribution, not the developer's potentially shared-library Node.
// Digests are pinned from https://nodejs.org/dist/v22.22.1/SHASUMS256.txt.
import { createHash, randomUUID } from 'node:crypto';
import { createReadStream, createWriteStream } from 'node:fs';
import { access, chmod, mkdir, mkdtemp, readFile, rename, rm, stat } from 'node:fs/promises';
import { pipeline } from 'node:stream/promises';
import { Readable, Transform } from 'node:stream';
import { promisify } from 'node:util';
import { execFile } from 'node:child_process';
import path from 'node:path';

const execute = promisify(execFile);
import nodeDistributions from './node-distributions.json' with { type: 'json' };
export const NODE_VERSION = nodeDistributions.version;
const DIGESTS = nodeDistributions.digests;
const MAX_ARCHIVE_BYTES = 128 * 1024 * 1024;

export function nodeDistribution(version, platform, architecture) {
  if (version !== NODE_VERSION) throw new Error('Update and verify the Node distribution digests when changing .node-version');
  const key = `${platform === 'win32' ? 'win' : platform}-${architecture}`;
  const sha256 = DIGESTS[key];
  if (!sha256) throw new Error(`Unsupported desktop Node distribution: ${platform}/${architecture}`);
  const directory = `node-v${version}-${key}`;
  const archive = `${directory}.${platform === 'win32' ? 'zip' : 'tar.gz'}`;
  return { version, platform, architecture, sha256, archive, directory,
    binary: platform === 'win32' ? 'node.exe' : 'bin/node',
    url: `https://nodejs.org/dist/v${version}/${archive}` };
}

export async function verifyArchive(file, expectedSha256) {
  const info = await stat(file);
  if (!info.isFile() || info.size === 0 || info.size > MAX_ARCHIVE_BYTES) throw new Error('Node archive has an invalid size');
  const hash = createHash('sha256');
  for await (const chunk of createReadStream(file)) hash.update(chunk);
  if (hash.digest('hex') !== expectedSha256) throw new Error('Node archive checksum mismatch; refusing to execute it');
}

async function downloadArchive(distribution, file) {
  const temporary = `${file}.${randomUUID()}.download`;
  try {
    const response = await fetch(distribution.url, { redirect: 'error', signal: AbortSignal.timeout(120_000) });
    if (!response.ok || !response.body) throw new Error(`Node download failed with HTTP ${response.status}`);
    let bytes = 0;
    const bounded = new Transform({ transform(chunk, encoding, callback) {
      bytes += chunk.length;
      callback(bytes > MAX_ARCHIVE_BYTES ? new Error('Node download exceeded size limit') : null, chunk);
    } });
    await pipeline(Readable.fromWeb(response.body), bounded, createWriteStream(temporary, { flags: 'wx', mode: 0o600 }));
    await verifyArchive(temporary, distribution.sha256);
    await rename(temporary, file);
  } finally {
    await rm(temporary, { force: true });
  }
}

export async function prepareNodeRuntime(repository, environment = process) {
  const version = (await readFile(path.join(repository, '.node-version'), 'utf8')).trim();
  const distribution = nodeDistribution(version, environment.platform, environment.arch);
  const cache = path.join(repository, 'target', 'node-distributions');
  await mkdir(cache, { recursive: true });
  // Optional offline input is still checked against the repository-pinned digest.
  const archive = environment.env.OMNISOLO_NODE_ARCHIVE
    ? path.resolve(environment.env.OMNISOLO_NODE_ARCHIVE) : path.join(cache, distribution.archive);
  try { await access(archive); }
  catch (error) {
    if (error.code !== 'ENOENT' || environment.env.OMNISOLO_NODE_ARCHIVE) throw error;
    await downloadArchive(distribution, archive);
  }
  await verifyArchive(archive, distribution.sha256);
  const unpacked = await mkdtemp(path.join(cache, `${distribution.directory}-`));
  try {
    // Extract only known members from an authenticated upstream archive. Never
    // execute or trust an unpacked executable from a previous cache entry.
    await execute('tar', ['-xf', archive, '--strip-components=1', '-C', unpacked,
      `${distribution.directory}/${distribution.binary}`, `${distribution.directory}/LICENSE`],
    { timeout: 30_000, maxBuffer: 1024 * 1024 });
    const binaryPath = path.join(unpacked, distribution.binary);
    await chmod(binaryPath, 0o755);
    const { stdout } = await execute(binaryPath, ['-p', 'JSON.stringify({version:process.versions.node,arch:process.arch,platform:process.platform,shared:!!process.config.variables.node_shared})'],
      { timeout: 10_000, maxBuffer: 4096, env: { PATH: environment.env.PATH, SystemRoot: environment.env.SystemRoot } });
    const identity = JSON.parse(stdout);
    if (identity.version !== version || identity.arch !== environment.arch || identity.platform !== environment.platform || identity.shared) {
      throw new Error('Node distribution identity or linkage does not match this desktop target');
    }
    return { binaryPath, licensePath: path.join(unpacked, 'LICENSE'), distribution,
      cleanup: () => rm(unpacked, { recursive: true, force: true }) };
  } catch (error) {
    await rm(unpacked, { recursive: true, force: true });
    throw error;
  }
}
