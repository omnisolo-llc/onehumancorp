import { NextResponse, NextRequest } from 'next/server';

export async function POST(request: NextRequest) {
  const backendUrl = process.env.OHC_AGENT_BUILTIN_SERVICE_URL || 'http://localhost:50051';
  // Note: OHC backend service is typically gRPC.
  // However, the Next.js API routes often proxy to a Go server that handles gRPC,
  // or use a gRPC-web gateway.
  // For this implementation, we'll assume the Next.js route proxies to the builtin agent service's
  // gRPC endpoint if configured, or a REST wrapper if available.

  // Since I don't see a clear REST-to-gRPC mapping in the current repo for this new RPC,
  // I will implement the route to match the expected format for the frontend.

  try {
    const body = await request.json();
    const { task_id, tool_call_id, input, resolution_type } = body;

    // Implementation Detail: In OHC, the builtin agent service is a Rust gRPC server.
    // The main Go server usually proxies these.
    // We'll call the backend endpoint that handles ResolveIntervention.

    const res = await fetch(`${backendUrl}/ohc.agent.service.AgentService/ResolveIntervention`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        task_id,
        tool_call_id,
        input,
        resolution_type,
      }),
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    const errorText = await res.text();
    return NextResponse.json({ error: errorText || 'Failed to resolve intervention' }, { status: res.status });
  } catch (e: any) {
    return NextResponse.json({ error: e.message || 'Intervention resolution proxy failed' }, { status: 500 });
  }
}
