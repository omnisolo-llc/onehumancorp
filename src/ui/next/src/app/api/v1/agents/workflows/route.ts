import { spawn } from 'node:child_process';
import { randomUUID } from 'node:crypto';
import { NextRequest, NextResponse } from 'next/server';

export const runtime = 'nodejs';

type WorkflowStatus = 'queued' | 'running' | 'completed' | 'failed';

type WorkflowRecord = {
  id: string;
  name: string;
  workflow: 'ohc_review_branch';
  task: string;
  status: WorkflowStatus;
  command: string;
  created_at: string;
  output?: string;
  error?: string;
};

const workflows: WorkflowRecord[] = [];

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

function buildAgentTask(task: string) {
  return [
    'Use the built-in RunWorkflow tool.',
    `Arguments: ${JSON.stringify({ workflow: 'ohc_review_branch', task })}.`,
    'Return the final synthesized report.',
  ].join(' ');
}

function dispatchWorkflow(record: WorkflowRecord) {
  const binary = agentBinary();
  const agentTask = buildAgentTask(record.task);

  record.status = 'running';
  record.command = `${binary} --task ${JSON.stringify(agentTask)}`;

  const child = spawn(binary, ['--task', agentTask], {
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

  child.on('error', (err) => {
    record.status = 'failed';
    record.error = `Failed to start ${binary}: ${err.message}`;
  });

  child.on('close', (code) => {
    if (record.status === 'failed') {
      return;
    }

    record.status = code === 0 ? 'completed' : 'failed';
    record.output = stdout.trim();
    if (code !== 0) {
      record.error = stderr.trim() || `Agent CLI exited with code ${code}`;
    }
  });
}

export async function GET() {
  return NextResponse.json({ workflows });
}

export async function POST(request: NextRequest) {
  const body = await request.json().catch(() => null);
  const name = typeof body?.name === 'string' ? body.name.trim() : '';
  const task = typeof body?.task === 'string' ? body.task.trim() : '';

  if (!name || !task) {
    return NextResponse.json({ error: 'Workflow name and task are required' }, { status: 400 });
  }

  const record: WorkflowRecord = {
    id: randomUUID(),
    name,
    workflow: 'ohc_review_branch',
    task,
    status: 'queued',
    command: '',
    created_at: new Date().toISOString(),
  };

  workflows.unshift(record);
  dispatchWorkflow(record);

  return NextResponse.json({ workflow: record }, { status: 202 });
}
