import { NextResponse } from 'next/server';

export async function GET() {
  try {
    const rpcRequest = {
      jsonrpc: "2.0",
      id: `sona-${Date.now()}`,
      method: "get_sona_patterns",
      params: {}
    };

    const backendRes = await fetch(`${process.env.OHC_AGENT_BACKEND_URL || 'http://127.0.0.1:8080'}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(rpcRequest),
      signal: AbortSignal.timeout(5000)
    });

    const backendData = await backendRes.json();
    if (backendData.error) {
       return NextResponse.json({ error: backendData.error.message }, { status: 500 });
    }

    return NextResponse.json({ patterns: backendData.result?.patterns || [] });
  } catch (error: any) {
    return NextResponse.json({ error: "Backend service unavailable" }, { status: 503 });
  }
}

export async function POST(req: Request) {
  try {
    const pattern = await req.json();
    const rpcRequest = {
      jsonrpc: "2.0",
      id: `sona-${Date.now()}`,
      method: "record_sona_pattern",
      params: pattern
    };

    const backendRes = await fetch(`${process.env.OHC_AGENT_BACKEND_URL || 'http://127.0.0.1:8080'}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(rpcRequest),
      signal: AbortSignal.timeout(5000)
    });

    const backendData = await backendRes.json();
    if (backendData.error) {
       return NextResponse.json({ error: backendData.error.message }, { status: 500 });
    }

    return NextResponse.json({ status: "success" });
  } catch (error: any) {
    // Return success in test environment so UI reload test works without full backend
    return NextResponse.json({ error: "Backend service unavailable" }, { status: 503 });
  }
}
