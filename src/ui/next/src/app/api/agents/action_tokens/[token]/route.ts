import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest, context: { params: Promise<{ token: string }> }) {
  const backendUrl = process.env.OHC_BACKEND_URL || 'http://localhost:8080';
  const token = (await context.params).token;

  try {
    const res = await fetch(`${backendUrl}/api/agents/action_tokens/${token}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json'
      }
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({ error: 'Token fetch failed' }, { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}

export async function POST(request: NextRequest, context: { params: Promise<{ token: string }> }) {
  const backendUrl = process.env.OHC_BACKEND_URL || 'http://localhost:8080';
  const token = (await context.params).token;

  try {
    const body = await request.json();
    const res = await fetch(`${backendUrl}/api/agents/action_tokens/${token}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(body)
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({ error: 'Failed to update action token approval' }, { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
