import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const agentUrl = process.env.OHC_AGENT_URL || 'http://127.0.0.1:18789';

  try {
    const { project_trusted, allowed_tools, high_risk_tools, tool_to_run } = await req.json();

    if (!tool_to_run) {
      return NextResponse.json({ error: 'tool_to_run is required' }, { status: 400 });
    }

    const rpcRequest = {
      jsonrpc: "2.0",
      id: "guardrails-1",
      method: "test_guardrails",
      params: {
        project_trusted: !!project_trusted,
        allowed_tools: allowed_tools || [],
        high_risk_tools: high_risk_tools || [],
        tool_to_run
      }
    };

    const backendRes = await fetch(`${agentUrl}/rpc`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(rpcRequest),
      signal: AbortSignal.timeout(10000)
    });

    const backendData = await backendRes.json();

    if (backendData.error) {
       return NextResponse.json({ error: backendData.error.message }, { status: 500 });
    }

    return NextResponse.json(backendData.result);
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}
