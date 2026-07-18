import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const agentUrl = process.env.OHC_AGENT_URL || 'http://127.0.0.1:18789';

  try {
    const { message } = await req.json();

    if (!message) {
      return NextResponse.json({ error: 'Message is required' }, { status: 400 });
    }

    const rpcRequest = {
      jsonrpc: "2.0",
      id: "actor-model-1",
      method: "run_actor_model",
      params: { message }
    };

    const backendRes = await fetch(`${agentUrl}/rpc`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(rpcRequest),
      signal: AbortSignal.timeout(60000)
    });

    const backendData = await backendRes.json();

    if (backendData.error) {
       return NextResponse.json({ error: backendData.error.message }, { status: 500 });
    }

    const resultText = backendData.result?.output || "Executed successfully with empty output.";

    return NextResponse.json({ result: resultText });
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}
