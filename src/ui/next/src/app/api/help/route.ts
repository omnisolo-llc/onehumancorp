import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${backendUrl}/api/help`);

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json([], { status: res.status });
  } catch (e) {
    if (process.env.NODE_ENV !== "test") console.error("Failed to fetch help from backend:", e);
    return NextResponse.json([], { status: 500 });
  }
}
