import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const q = searchParams.get('q') || '';

  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';

  try {
    const res = await fetch(`${backendUrl}/api/help/search?q=${encodeURIComponent(q)}`);

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json([], { status: res.status });
  } catch (e) {
    if (process.env.NODE_ENV !== "test") console.error("Failed to fetch help search from backend:", e);
    return NextResponse.json([], { status: 500 });
  }
}
