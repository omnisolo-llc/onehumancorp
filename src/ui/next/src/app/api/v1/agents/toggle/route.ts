import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { agent, enabled } = await req.json();

    if (!agent || typeof enabled !== 'boolean') {
      return NextResponse.json({ error: 'Missing required fields' }, { status: 400 });
    }

    const backendUrl = process.env.OHC_CORE_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/agents/toggle`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ agent, enabled }),
    });

    if (!backendRes.ok) {
       return NextResponse.json({ error: 'Failed to toggle agent' }, { status: 500 });
    }

    const data = await backendRes.json();
    return NextResponse.json(data);
  } catch (error) {
    return NextResponse.json({ error: 'Failed to toggle agent' }, { status: 500 });
  }
}
