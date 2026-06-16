import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { message } = await req.json();

    if (!message) {
      return NextResponse.json({ error: 'message is required' }, { status: 400 });
    }

    const rpcRequest = {
      jsonrpc: "2.0",
      id: "langgraph-1",
      method: "run_agent",
      params: {
        agent_id: "default",
        message,
        config: {
          enable_langgraph_mechanic: true
        }
      }
    };

    let resultData = null;

    try {
      const backendRes = await fetch(process.env.AGENT_SERVICE_URL || 'http://127.0.0.1:8080', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(rpcRequest),
        signal: AbortSignal.timeout(10000)
      });

      const backendData = await backendRes.json();

      if (backendData.error) {
         return NextResponse.json({ error: backendData.error.message }, { status: 500 });
      }

      resultData = backendData.result;
    } catch (e: any) {
      return NextResponse.json({ error: e.message || "Backend service unavailable" }, { status: 503 });
    }

    return NextResponse.json({ result: resultData });
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}
