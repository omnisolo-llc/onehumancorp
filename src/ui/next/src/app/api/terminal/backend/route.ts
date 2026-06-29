import { NextResponse } from 'next/server';

const backendUrl = process.env.OHC_API_URL || 'http://127.0.0.1:18789';

export async function GET(request: Request) {
  try {
    const res = await fetch(`${backendUrl}/api/v1/payments/terminal/backend`, {
      method: 'GET',
    });
    if (!res.ok) {
        return NextResponse.json({ backend: 'local' });
    }
    const data = await res.json();
    return NextResponse.json({ backend: data.backend || 'local' });
  } catch (err: any) {
    return NextResponse.json({ backend: 'local' });
  }
}

export async function POST(request: Request) {
  try {
    const { backend } = await request.json();
    if (backend === 'local' || backend === 'docker') {
      try {
        const res = await fetch(`${backendUrl}/api/v1/payments/terminal/backend`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ backend }),
        });
        if (res.ok) {
            const data = await res.json();
            return NextResponse.json({ success: true, backend: data.backend || backend });
        }
      } catch (e) {
          console.warn("Backend failed", e);
      }
      return NextResponse.json({ success: true, backend });
    }
    return NextResponse.json({ error: 'Invalid backend' }, { status: 400 });
  } catch (err: any) {
    return NextResponse.json({ error: err.message }, { status: 500 });
  }
}
