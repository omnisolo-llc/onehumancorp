import { spawn } from 'node:child_process';
import { NextRequest, NextResponse } from 'next/server';

export const runtime = 'nodejs';

function agentBinary() {
  const override = process.env.OHC_BUILTIN_AGENT_BINARY || process.env.OHC_AGENT_BINARY;
  if (override) {
    return override;
  }

  const standaloneMode = (process.env.OHC_STANDALONE_MODE || '').toLowerCase();
  const sourceMode = (process.env.OHC_SOURCE_MODE || '').toLowerCase();
  const isCluster = standaloneMode === 'false' || ['cloud', 'cluster', 'headless'].includes(sourceMode);

  return isCluster ? 'ohc-builtin-agent' : 'server';
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();

    const binary = agentBinary();
    const taskData = `Execute block-based visual workflow: ${JSON.stringify(body)}`;

    const child = spawn(binary, ['--task', taskData], {
      env: process.env,
      stdio: ['ignore', 'pipe', 'pipe'],
      detached: false,
    });

    let stdout = '';
    let stderr = '';

    child.stdout.on('data', (chunk) => {
      stdout += chunk.toString();
    });

    child.stderr.on('data', (chunk) => {
      stderr += chunk.toString();
    });

    return new Promise((resolve) => {
      child.on('close', (code) => {
        if (code !== 0) {
          resolve(NextResponse.json({ error: stderr || \`Process exited with code \${code}\` }, { status: 500 }));
        } else {
          resolve(NextResponse.json({ result: stdout.trim() }));
        }
      });
      child.on('error', (err) => {
         resolve(NextResponse.json({ error: err.message }, { status: 500 }));
      });
    });
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 400 });
  }
}
