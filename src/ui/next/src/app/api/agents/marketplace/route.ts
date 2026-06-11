import { NextRequest, NextResponse } from 'next/server';

export const runtime = 'nodejs';

async function proxyToAgent(method: string, params: any) {
  const agentUrl = process.env.OHC_AGENT_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${agentUrl}/json_rpc`, {
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

    throw new Error(`Failed to call agent RPC: ${res.status}`);
  } catch (e: any) {
    throw e;
  }
}

export async function GET(request: NextRequest) {
  const method = request.nextUrl.searchParams.get('method');

  try {
    if (method === 'fetch') {
      const agent_id = request.nextUrl.searchParams.get('agent_id');
      const result = await proxyToAgent('am_fetch_agent', { agent_id });
      return NextResponse.json(result);
    } else {
      const q = request.nextUrl.searchParams.get('q') || '';
      const result = await proxyToAgent('am_search_agents', { query: q });
      return NextResponse.json(result);
    }
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}
