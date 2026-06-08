import { NextResponse, NextRequest } from 'next/server';

export const dynamic = 'force-dynamic';
export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${backendUrl}/api/changelog`);

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json([], { status: res.status });
  } catch (e) {
    if (process.env.NODE_ENV !== "test") console.error("Failed to fetch changelog from backend:", e);
    return NextResponse.json([], { status: 500 });
  }
}
