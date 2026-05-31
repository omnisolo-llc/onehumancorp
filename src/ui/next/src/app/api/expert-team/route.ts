import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { task } = await req.json();

    if (!task) {
      return NextResponse.json({ error: 'Task is required' }, { status: 400 });
    }

    const rpcRequest = {
      jsonrpc: "2.0",
      id: "expert-team-1",
      method: "run_expert_team",
      params: { message: task }
    };

    let resultText = "";

    try {
      const backendRes = await fetch('http://127.0.0.1:8080', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(rpcRequest),
        signal: AbortSignal.timeout(10000)
      });

      const backendData = await backendRes.json();

      if (backendData.error) {
         return NextResponse.json({ error: backendData.error.message }, { status: 500 });
      }

      resultText = backendData.result?.output || "Executed successfully with empty output.";
    } catch (e) {
      return NextResponse.json({ error: "Backend service unavailable" }, { status: 503 });
    }

    return NextResponse.json({ result: resultText });
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}
