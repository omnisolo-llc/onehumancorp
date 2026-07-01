import { randomUUID } from 'node:crypto';
import { NextRequest, NextResponse } from 'next/server';

export const runtime = 'nodejs';

type HireRequest = {
  name?: string;
  role?: string;
  providerType?: string;
  model?: string;
  mode?: string;
  workspace?: string;
  task?: string;
  skills?: string[];
  connectors?: string[];
  contextReferences?: string;
  attachments?: string;
  customProvider?: string;
  workDirectory?: string;
  outputFormat?: string;
  taskConstraints?: string;
};

function backendUrl() {
  return process.env.OHC_API_URL || process.env.BACKEND_URL || '';
}

export async function POST(request: NextRequest) {
  const payload = (await request.json().catch(() => null)) as HireRequest | null;
  const name = typeof payload?.name === 'string' ? payload.name.trim() : '';
  const role = typeof payload?.role === 'string' ? payload.role.trim() : '';

  if (!name || !role) {
    return NextResponse.json({ status: 'error', message: 'Expert name and role are required' }, { status: 400 });
  }

  const upstream = backendUrl();
  if (!upstream) {
    return NextResponse.json(
      { status: 'error', message: 'Backend hire service unavailable' },
      { status: 503 },
    );
  }

  try {
    const response = await fetch(`${upstream}/api/agents/hire`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        name,
        role,
        providerType: payload?.providerType || 'builtin',
        model: payload?.model || 'MiniMax-M3',
        mode: payload?.mode || 'Ask',
        workspace: payload?.workspace || 'Current business',
        task: payload?.task || '',
        skills: payload?.skills || [],
        connectors: payload?.connectors || [],
        contextReferences: payload?.contextReferences || '',
        attachments: payload?.attachments || '',
        customProvider: payload?.customProvider || '',
        workDirectory: payload?.workDirectory || '',
        outputFormat: payload?.outputFormat || 'Brief',
        taskConstraints: payload?.taskConstraints || '',
      }),
      signal: AbortSignal.timeout(15_000),
    });
    const data = await response.json().catch(() => null);
    return NextResponse.json(data || {}, { status: response.status });
  } catch (error) {
    return NextResponse.json(
      { status: 'error', message: 'Backend hire service unavailable' },
      { status: 503 },
    );
  }
}
