import { cp, mkdir, readFile, rm, chmod, writeFile } from 'node:fs/promises';
import { spawn } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { access } from 'node:fs/promises';
import { distributablePath, validateWebArtifact } from './package-web.mjs';
import { prepareNodeRuntime } from './node-runtime.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
export async function prepareDesktop() {
  const pinnedNode = (await readFile(path.join(root, '.node-version'), 'utf8')).trim();
  if (process.versions.node !== pinnedNode) throw new Error(`Desktop packaging requires Node ${pinnedNode}; found ${process.versions.node}`);
  if (process.env.OMNISOLO_DESKTOP_RESOURCES_PREPARED === '1') {
    const prepared = path.join(root, 'src/ui/tauri/native-resources');
    await validateWebArtifact(path.join(prepared, 'web'), root);
    await access(path.join(prepared, 'bin', process.platform === 'win32' ? 'node.exe' : 'node'));
    await access(path.join(prepared, 'web/src/ui/next/native-entry.cjs'));
    await access(path.join(prepared, 'NODE-LICENSE'));
    await access(path.join(prepared, 'node-distribution.json'));
    return; // Preserve a Node executable signed by the release job.
  }
  const prebuilt = process.env.OMNISOLO_PREBUILT_WEB;
  if (!prebuilt) await new Promise((resolve, reject) => {
    const child = spawn(process.platform === 'win32' ? 'npm.cmd' : 'npm', ['run', 'build:web'], {
      cwd: root, stdio: 'inherit', shell: process.platform === 'win32',
    });
    child.once('error', reject); child.once('exit', (code) => code === 0 ? resolve() : reject(new Error(`Native web build failed (${code})`)));
  });
  const source = prebuilt ? path.resolve(root, prebuilt) : path.join(root, 'target/native-web');
  const manifest = await validateWebArtifact(source, root);
  const destination = path.join(root, 'src/ui/tauri/native-resources');
  await rm(destination, { recursive: true, force: true });
  await mkdir(path.join(destination, 'bin'), { recursive: true });
  await cp(source, path.join(destination, 'web'), { recursive: true, dereference: true, filter: distributablePath });
  const nodeName = process.platform === 'win32' ? 'node.exe' : 'node';
  const runtime = await prepareNodeRuntime(root);
  try {
    await cp(runtime.binaryPath, path.join(destination, 'bin', nodeName));
    await chmod(path.join(destination, 'bin', nodeName), 0o755);
    await cp(runtime.licensePath, path.join(destination, 'NODE-LICENSE'));
    await writeFile(path.join(destination, 'node-distribution.json'), JSON.stringify(runtime.distribution, null, 2) + '\n');
  } finally { await runtime.cleanup(); }
  await cp(path.join(root, 'scripts/native-web-entry.cjs'), path.join(destination, 'web/src/ui/next/native-entry.cjs'));
  await writeFile(path.join(destination, 'runtime-manifest.json'), JSON.stringify({
    ...manifest, nodeExecutable: `bin/${nodeName}`, entry: 'web/src/ui/next/native-entry.cjs',
  }, null, 2) + '\n');
}
