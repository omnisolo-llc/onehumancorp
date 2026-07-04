import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  const agentUrl = process.env.OHC_AGENT_URL || 'http://127.0.0.1:18789';

  try {
    const body = await request.json();
    const { task_description } = body;

    if (!task_description) {
      return NextResponse.json({ error: 'task_description is required' }, { status: 400 });
    }

    const rpcRequest = {
      jsonrpc: "2.0",
      id: "crewai-1",
      method: "run_crewai",
      params: { task_description }
    };

    const startTime = Date.now();

    const backendRes = await fetch(`${agentUrl}/rpc`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(rpcRequest),
      signal: AbortSignal.timeout(60000)
    });

    const execution_time_ms = Date.now() - startTime;
    const backendData = await backendRes.json();

    if (backendData.error) {
       return NextResponse.json({ error: backendData.error.message }, { status: 500 });
    }

    return NextResponse.json({
      status: 'success',
      report: backendData.result?.report || "Executed successfully with empty output.",
      execution_time_ms,
    });
  } catch (error: any) {
    return NextResponse.json({ error: `Failed to execute CrewAI workflow: ${error.message}` }, { status: 500 });
  }
}
