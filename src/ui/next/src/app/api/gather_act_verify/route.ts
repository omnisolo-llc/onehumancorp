import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();

    // Use OHC_AGENT_URL for backend communication as standard
    const agentUrl = process.env.OHC_AGENT_URL || "http://127.0.0.1:18789";

    const response = await fetch(`${agentUrl}/api/agent/gather_act_verify`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      const errorText = await response.text();
      return NextResponse.json({ error: errorText }, { status: response.status });
    }

    const data = await response.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error('Error in Gather Act Verify proxy route:', error);
    return NextResponse.json(
      { error: 'Failed to communicate with Gather Act Verify backend' },
      { status: 500 }
    );
  }
}
