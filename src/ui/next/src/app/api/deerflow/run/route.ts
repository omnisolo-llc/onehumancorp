import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { task } = await req.json();

    if (!task) {
      return NextResponse.json({ error: 'Task is required' }, { status: 400 });
    }

    const rpcRequest = {
      jsonrpc: "2.0",
      id: "deerflow-1",
      method: "run_deerflow_orchestration",
      params: { task }
    };

    let resultText = "";

    try {
      const backendUrl = process.env.OHC_API_URL || 'http://127.0.0.1:8080';
      const backendRes = await fetch(`${backendUrl}/rpc`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(rpcRequest),
        signal: AbortSignal.timeout(30000)
      });

      const backendData = await backendRes.json();

      if (backendData.error) {
         return NextResponse.json({ error: backendData.error.message }, { status: 500 });
      }

      resultText = backendData.result?.output || backendData.result || "Executed successfully with empty output.";
    } catch (e: any) {
      return NextResponse.json({ error: `Backend service unavailable: ${e.message}` }, { status: 503 });
    }

    return NextResponse.json({ result: resultText });
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}
