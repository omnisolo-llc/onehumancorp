import { cp, mkdir, readFile, readdir, rm, writeFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { fileURLToPath, pathToFileURL } from 'node:url';
import path from 'node:path';
import { createHash } from 'node:crypto';

export const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
export function webLayout(repository = root) {
  return {
    source: path.join(repository, 'src/ui/next'),
    standalone: path.join(repository, 'src/ui/next/.next/standalone'),
    destination: path.join(repository, 'target/native-web'),
    relativeServer: 'src/ui/next/server.js',
  };
}

export function distributablePath(source) {
  return !source.split(path.sep).some((part) => /^\.env(?:\.|$)/.test(part))
    && !source.includes(`${path.sep}.next${path.sep}cache`);
}

// Bind reusable web artifacts to their actual source, not just dependency locks.
// Exclude generated Next declarations and local secrets from both digest and bundle.
export async function sourceDigest(repository = root) {
  const hash = createHash('sha256');
  const source = path.join(repository, 'src/ui/next');
  async function visit(directory) {
    const entries = (await readdir(directory, { withFileTypes: true })).sort((a, b) => a.name.localeCompare(b.name, 'en'));
    for (const entry of entries) {
      if (['node_modules', '.next', '.git', 'coverage', 'next-env.d.ts'].includes(entry.name)
          || entry.name.endsWith('.tsbuildinfo') || /^\.env(?:\.|$)/.test(entry.name)) continue;
      const file = path.join(directory, entry.name);
      if (entry.isDirectory()) await visit(file);
      else if (entry.isFile()) {
        hash.update(path.relative(repository, file).split(path.sep).join('/') + '\0');
        hash.update(await readFile(file));
        hash.update('\0');
      } else if (entry.isSymbolicLink()) throw new Error('Frontend source symlinks require an explicit packaging policy');
    }
  }
  await visit(source);
  for (const name of ['package.json', 'package-lock.json', '.node-version', 'scripts/build-web.mjs', 'scripts/package-web.mjs', 'scripts/native-web-entry.cjs']) {
    if (existsSync(path.join(repository, name))) {
      hash.update(name + '\0'); hash.update(await readFile(path.join(repository, name))); hash.update('\0');
    }
  }
  return hash.digest('hex');
}

export async function validateWebArtifact(source, repository = root, environment = process) {
  const manifest = JSON.parse(await readFile(path.join(source, 'build-manifest.json'), 'utf8'));
  const lock = await readFile(path.join(repository, 'src/ui/next/package-lock.json'));
  if (manifest.schemaVersion !== 2 || manifest.platform !== environment.platform
      || manifest.architecture !== environment.arch || manifest.node !== environment.versions.node
      || manifest.packageLockSha256 !== createHash('sha256').update(lock).digest('hex')
      || manifest.sourceSha256 !== await sourceDigest(repository)
      || manifest.server !== webLayout(repository).relativeServer
      || !existsSync(path.join(source, manifest.server))) {
    throw new Error('Prebuilt web artifact does not match this source, platform, architecture, Node version and dependency lock');
  }
  return manifest;
}

// Called only after a successful compiler exit by build-web.mjs. Reject an
// edit made while the build ran; packaging may not bless an old .next directory
// with a newly calculated source digest.
export async function recordSuccessfulBuild(expectedSource, repository = root) {
  const layout = webLayout(repository);
  const buildId = (await readFile(path.join(layout.source, '.next/BUILD_ID'), 'utf8')).trim();
  if (!buildId || expectedSource !== await sourceDigest(repository)) {
    throw new Error('Frontend source changed during build; rebuild before packaging');
  }
  const proof = { schemaVersion: 2, buildId, sourceSha256: expectedSource,
    node: process.versions.node, platform: process.platform, architecture: process.arch };
  await writeFile(path.join(layout.source, '.next/ohc-build-proof.json'), JSON.stringify(proof) + '\n');
}

export async function packageWeb(repository = root) {
  const layout = webLayout(repository);
  const buildId = (await readFile(path.join(layout.source, '.next/BUILD_ID'), 'utf8')).trim();
  if (!buildId || !existsSync(path.join(layout.standalone, layout.relativeServer))) {
    throw new Error('A successful fresh Next standalone build is required; run npm run build:web.');
  }
  let proof;
  try { proof = JSON.parse(await readFile(path.join(layout.source, '.next/ohc-build-proof.json'), 'utf8')); }
  catch { throw new Error('Fresh native build proof is missing; run npm run build:web, not the packager alone'); }
  const digest = await sourceDigest(repository);
  if (proof.schemaVersion !== 2 || proof.buildId !== buildId || proof.sourceSha256 !== digest
      || proof.node !== process.versions.node || proof.platform !== process.platform || proof.architecture !== process.arch) {
    throw new Error('Stale or foreign Next build cannot be repackaged as current source');
  }
  // Never merge old release output into a new bundle. Keep .next/cache in the
  // source build directory, not in the shipped artifact.
  await rm(layout.destination, { recursive: true, force: true });
  await mkdir(layout.destination, { recursive: true });
  await cp(layout.standalone, layout.destination, {
    recursive: true, dereference: true,
    filter: (source) => distributablePath(source) && existsSync(source),
  });
  const app = path.join(layout.destination, 'src/ui/next');
  await cp(path.join(layout.source, '.next/static'), path.join(app, '.next/static'), {
    recursive: true, filter: distributablePath,
  });
  if (existsSync(path.join(layout.source, 'public'))) {
    await cp(path.join(layout.source, 'public'), path.join(app, 'public'), {
      recursive: true, filter: distributablePath,
    });
  }
  const lock = await readFile(path.join(layout.source, 'package-lock.json'));
  await writeFile(path.join(layout.destination, 'build-manifest.json'), JSON.stringify({
    schemaVersion: 2, buildId, node: process.versions.node,
    platform: process.platform, architecture: process.arch,
    packageLockSha256: createHash('sha256').update(lock).digest('hex'),
    sourceSha256: digest,
    server: layout.relativeServer,
  }, null, 2) + '\n');
  if (digest !== await sourceDigest(repository)) {
    await rm(layout.destination, { recursive: true, force: true });
    throw new Error('Frontend source changed during packaging; refusing mixed output');
  }
  console.log(`Packaged Next server ${buildId} at ${layout.destination}`);
  return layout;
}

if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) {
  packageWeb().catch((error) => { console.error(error.message); process.exitCode = 1; });
}
