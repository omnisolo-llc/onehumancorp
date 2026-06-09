import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const q = searchParams.get('q') || '';

  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  const authHeader = request.headers.get('authorization') || request.headers.get('cookie') || '';

  try {
    const res = await fetch(`${backendUrl}/api/v1/search?q=${encodeURIComponent(q)}`, {
        headers: {
            'cookie': authHeader
        }
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({ results: [] }, { status: res.status });
  } catch (e) {
    if (process.env.NODE_ENV !== "test") console.error("Failed to fetch search from backend:", e);
    return NextResponse.json({ results: [] }, { status: 500 });
  }
}
