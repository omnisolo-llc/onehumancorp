import { NextResponse } from 'next/server';
import { FaultInjector } from '../../../lib/chaos';

export async function POST(req: Request) {
  try {
    await FaultInjector.applyFault('expert_team_api_start');
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
      await FaultInjector.applyFault('expert_team_api_fetch_before');

      const backendRes = await fetch(process.env.OHC_AGENT_URL || 'http://127.0.0.1:18789/rpc', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(rpcRequest),
        signal: AbortSignal.timeout(60000)
      });

      await FaultInjector.applyFault('expert_team_api_fetch_after');

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
