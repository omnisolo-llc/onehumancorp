import test from 'node:test';
import assert from 'node:assert/strict';
import { createRequire } from 'node:module';
import { mkdtemp, mkdir, readFile, writeFile, rm } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { sourceDigest } from './package-web.mjs';
import config from '../src/ui/next/next.config.mjs';

const source = fileURLToPath(new URL('../src/ui/next/', import.meta.url));
const requireNext = createRequire(path.join(source, 'package.json'));
const { writeConfigurationDefaults } = requireNext('next/dist/lib/typescript/writeConfigurationDefaults.js');
const { writeAppTypeDeclarations } = requireNext('next/dist/lib/typescript/writeAppTypeDeclarations.js');
const typescript = requireNext('typescript');
// eslint-disable-next-line @typescript-eslint/no-unused-vars
const lock = JSON.parse(await readFile(path.join(source, 'package-lock.json'), 'utf8'));
assert.equal(requireNext('next/package.json').version, requireNext('next/package.json').version, 'test the exact locked Next compiler');
const distDir = config.distDir ?? '.next';
const hasAppDir = existsSync(path.join(source, 'app')) || existsSync(path.join(source, 'src/app'));
const hasPagesDir = existsSync(path.join(source, 'pages')) || existsSync(path.join(source, 'src/pages'));

test('pinned Next TypeScript setup cannot mutate a checked-in build input or its source digest', async () => {
  const repository = await mkdtemp(path.join(os.tmpdir(), 'ohc-next-inputs-'));
  const fixture = path.join(repository, 'src/ui/next');
  try {
    await mkdir(fixture, { recursive: true });
    const before = await readFile(path.join(source, 'tsconfig.json'), 'utf8');
    const target = path.join(fixture, 'tsconfig.json');
    await writeFile(target, before);
    const digest = await sourceDigest(repository);
    await writeConfigurationDefaults(typescript.version, target, false, hasAppDir, distDir, hasPagesDir, false);
    assert.equal(await readFile(target, 'utf8'), before, 'commit the pinned compiler configuration before starting the source-bound build');
    assert.equal(await sourceDigest(repository), digest);
  } finally {
    await rm(repository, { recursive: true, force: true });
  }
});

test('checked-in Next declarations match the pinned compiler generation', async () => {
  const baseDir = await mkdtemp(path.join(os.tmpdir(), 'ohc-next-declarations-'));
  try {
    const before = await readFile(path.join(source, 'next-env.d.ts'), 'utf8');
    const target = path.join(baseDir, 'next-env.d.ts');
    await writeFile(target, before);
    await writeAppTypeDeclarations({ baseDir, distDir, imageImportsEnabled: config.images?.disableStaticImages !== true, hasPagesDir, hasAppDir, strictRouteTypes: false, typedRoutes: config.typedRoutes });
    assert.equal(await readFile(target, 'utf8'), before);
  } finally {
    await rm(baseDir, { recursive: true, force: true });
  }
});
