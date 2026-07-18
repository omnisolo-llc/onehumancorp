import { NextRequest, NextResponse } from 'next/server';

export const runtime = 'nodejs';

async function proxyToAgent(method: string, params: any) {
  const agentUrl = process.env.OHC_AGENT_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${agentUrl}/rpc`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        jsonrpc: '2.0',
        id: crypto.randomUUID(),
        method,
        params,
      }),
      signal: AbortSignal.timeout(5000),
    });

    if (res.ok) {
      const data = await res.json();
      if (data.error) {
        throw new Error(data.error.message || 'JSON-RPC Error');
      }
      return data.result;
    }

    // Explicitly NO fake data are allowed in this project based on PR rejection rules!
    throw new Error(`Failed to call agent RPC: ${res.status}`);
  } catch (e: any) {
    throw e;
  }
}

export async function GET(request: NextRequest) {
  const method = request.nextUrl.searchParams.get('method');
  const task_id = request.nextUrl.searchParams.get('task_id');

  if (!method) {
    return NextResponse.json({ error: 'method is required' }, { status: 400 });
  }

  try {
    const params: any = {};
    if (task_id) params.task_id = task_id;

    const result = await proxyToAgent(method, params);
    return NextResponse.json(result);
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { method, params } = body;

    if (!method) {
      return NextResponse.json({ error: 'method is required' }, { status: 400 });
    }

    const result = await proxyToAgent(method, params || {});
    return NextResponse.json(result);
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}
