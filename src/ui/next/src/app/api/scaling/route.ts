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
      signal: AbortSignal.timeout(60000),
    });

    if (res.ok) {
      const data = await res.json();
      if (data.error) {
        throw new Error(data.error.message || 'JSON-RPC Error');
      }
      return data.result;
    }

    throw new Error(`Failed to call agent RPC: ${res.status}`);
  } catch (e: any) {
    throw e;
  }
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { count, message } = body;

    const result = await proxyToAgent('run_scalable_agents', { count, message });
    return NextResponse.json(result);
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}
