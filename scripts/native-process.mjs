import { spawn, spawnSync } from 'node:child_process';
import path from 'node:path';

const TAIL_LIMIT = 128 * 1024;
function retainTail(previous, chunk) {
  if (chunk.length >= TAIL_LIMIT) return Buffer.from(chunk.subarray(chunk.length - TAIL_LIMIT));
  const combined = Buffer.concat([previous, chunk]);
  return combined.length > TAIL_LIMIT ? Buffer.from(combined.subarray(combined.length - TAIL_LIMIT)) : combined;
}

// All processes are owned children, launched with literal argv. Timeouts and
// cancellation are failures even when a child catches SIGTERM and exits zero.
export function runNativeCommand(executable, args, {
  cwd, env, input, quiet = false, timeoutMs = 240_000, signal,
} = {}) {
  if (!Number.isSafeInteger(timeoutMs) || timeoutMs < 1 || timeoutMs > 7_200_000) {
    return Promise.reject(new TypeError('Invalid native command deadline'));
  }
  if (signal?.aborted) return Promise.reject(new Error('Native command cancelled before launch'));
  return new Promise((resolve, reject) => {
    const child = spawn(executable, args, { cwd, env, shell: false,
      detached: process.platform !== 'win32', windowsHide: true, stdio: ['pipe', 'pipe', 'pipe'] });
    let stdout = Buffer.alloc(0), stderr = Buffer.alloc(0), failure, closed = false, killTimer;
    const kill = (force) => {
      if (closed || !child.pid) return;
      if (process.platform === 'win32') {
        spawnSync('taskkill', ['/pid', String(child.pid), '/T', '/F'], { windowsHide: true, timeout: 5000, stdio: 'ignore' });
      } else {
        try { process.kill(-child.pid, force ? 'SIGKILL' : 'SIGTERM'); }
        catch (error) { if (error.code !== 'ESRCH') child.kill(force ? 'SIGKILL' : 'SIGTERM'); }
      }
    };
    const stop = (reason) => {
      if (failure || closed) return;
      failure = new Error(`${path.basename(executable)} ${reason}`);
      kill(false);
      killTimer = setTimeout(() => kill(true), 2000);
      killTimer.unref();
    };
    const deadline = setTimeout(() => stop(`exceeded its ${timeoutMs}ms deadline`), timeoutMs);
    deadline.unref();
    const abort = () => stop('was cancelled');
    signal?.addEventListener('abort', abort, { once: true });
    // Close the small race between the preflight abort check and listener setup.
    if (signal?.aborted) abort();
    child.stdout.on('data', chunk => { stdout = retainTail(stdout, chunk); if (!quiet) process.stdout.write(chunk); });
    child.stderr.on('data', chunk => { stderr = retainTail(stderr, chunk); if (!quiet) process.stderr.write(chunk); });
    child.once('error', error => { failure ??= new Error(`${path.basename(executable)} could not start`, { cause: error }); });
    child.once('close', (code, exitSignal) => {
      // A cooperative parent may exit while a descendant ignores SIGTERM.
      // Finish terminating only the process group created for this command.
      if (failure) kill(true);
      closed = true;
      clearTimeout(deadline); clearTimeout(killTimer);
      signal?.removeEventListener('abort', abort);
      if (failure) reject(failure);
      else if (code === 0) resolve(stdout.toString('utf8').trim());
      else reject(new Error(`${path.basename(executable)} failed (${exitSignal || code}): ${stderr.toString('utf8').slice(-2000)}`));
    });
    child.stdin.on('error', error => {
      // A child may reject input and close its pipe; the exit status is the
      // authoritative failure. Other pipe failures cannot become false success.
      if (error.code !== 'EPIPE' && error.code !== 'ECONNRESET') stop('input transport failed');
    });
    child.stdin.end(input);
  });
}
