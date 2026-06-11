import { NextResponse } from 'next/server';
import { FaultInjector } from '../../../lib/chaos';

export async function POST(req: Request) {
  try {
    await FaultInjector.applyFault('ralph_loop_api_start');
    const { task, progress_file } = await req.json();

    if (!task) {
      return NextResponse.json({ error: 'Task is required' }, { status: 400 });
    }

    const rpcRequest = {
      jsonrpc: "2.0",
      id: "ralph-loop-1",
      method: "run_ralph_loop",
      params: {
        task,
        progress_file: progress_file || ".ralph_progress.json"
      }
    };

    let resultData = null;

    try {
      await FaultInjector.applyFault('ralph_loop_api_fetch_before');

      const backendRes = await fetch('http://127.0.0.1:8080', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(rpcRequest),
        signal: AbortSignal.timeout(120000)
      });

      await FaultInjector.applyFault('ralph_loop_api_fetch_after');

      const backendData = await backendRes.json();

      if (backendData.error) {
         return NextResponse.json({ error: backendData.error.message }, { status: 500 });
      }

      resultData = backendData.result;
    } catch (e: any) {
      return NextResponse.json({ error: "Backend service unavailable: " + e.message }, { status: 503 });
    }

    return NextResponse.json({ result: resultData });
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}
