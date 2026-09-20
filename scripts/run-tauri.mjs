import { createRequire } from 'node:module';
import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import { prepareDesktop } from './prepare-desktop.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const require = createRequire(path.join(root, 'package.json'));
const pkg = require.resolve('@tauri-apps/cli/package.json');
const cli = path.join(path.dirname(pkg), 'tauri.js');
const args = process.argv.slice(2);
const mobile = args[0] === 'android' || args[0] === 'ios';
if (mobile && ['build', 'dev'].includes(args[1])) {
  args.push('--config', JSON.stringify({ bundle: { resources: [], createUpdaterArtifacts: false } }));
}
if (mobile && args.includes('build')) {
  const url = new URL(process.env.OMNISOLO_MOBILE_WEB_URL || 'about:blank');
  if (url.protocol !== 'https:' || url.username || url.password || url.search || url.hash) {
    throw new Error('Mobile builds require an explicit HTTPS OMNISOLO_MOBILE_WEB_URL; the Node web server runs on the configured host, not the phone.');
  }
}
if (!mobile && ['dev', 'build'].includes(args[0])) {
  const targetIndex = args.indexOf('--target');
  if (targetIndex !== -1) {
    const target = args[targetIndex + 1] || '';
    const architecture = target.split('-')[0];
    if ((architecture === 'aarch64' && process.arch !== 'arm64')
        || (architecture === 'x86_64' && process.arch !== 'x64') || architecture === 'universal') {
      throw new Error('Desktop Node resources must be built on the target architecture; use the matching native CI runner.');
    }
  }
  await prepareDesktop();
}
const child = spawn(process.execPath, [cli, ...args], {
  cwd: path.join(root, 'src/ui/tauri'), stdio: 'inherit', shell: false,
});
child.on('error', (error) => { console.error(error.message); process.exitCode = 1; });
child.on('exit', (code, signal) => { process.exitCode = signal ? 1 : code ?? 1; });
for (const signal of ['SIGINT', 'SIGTERM']) process.on(signal, () => child.kill(signal));
