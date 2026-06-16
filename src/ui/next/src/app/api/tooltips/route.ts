import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${backendUrl}/api/tooltips`).catch(() => null);

    if (res && res.ok) {
      const data = await res.json();
      if (data && Object.keys(data).length > 0) {
          return NextResponse.json(data);
      }
    }

    return NextResponse.json({ error: 'Backend failed' }, { status: 500 });
  } catch (e) {
    if (process.env.NODE_ENV !== "test") console.error("Failed to fetch tooltips from backend:", e);
    return NextResponse.json({ error: 'Backend failed' }, { status: 500 });
  }
}
