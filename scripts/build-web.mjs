import { spawn } from 'node:child_process';
import { rm } from 'node:fs/promises';
import path from 'node:path';
import { root, sourceDigest, recordSuccessfulBuild, packageWeb, webLayout } from './package-web.mjs';

// npm exposes its real JavaScript CLI path on every supported operating system.
// Executing that path with Node avoids Windows .cmd shell interpolation.
const npmCli = process.env.npm_execpath;
if (!npmCli) throw new Error('Run this build through npm run build:web');
const before = await sourceDigest(root);
await rm(path.join(webLayout(root).source, '.next/ohc-build-proof.json'), { force: true });
await new Promise((resolve, reject) => {
  const child = spawn(process.execPath, [npmCli, '--prefix', 'src/ui/next', 'run', 'build'],
    { cwd: root, stdio: 'inherit', shell: false });
  child.once('error', reject);
  child.once('exit', (code, signal) => code === 0 ? resolve() : reject(new Error(`Next build failed: ${signal || code}`)));
});
await recordSuccessfulBuild(before, root);
await packageWeb(root);
