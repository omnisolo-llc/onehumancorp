import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${backendUrl}/api/videos`).catch(() => null);

    if (res && res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json([], { status: res?.status || 500 });
  } catch (e) {
    if (process.env.NODE_ENV !== "test" && process.env.CI !== "1") {
      console.error("Failed to fetch videos from backend:", e);
    }
    return NextResponse.json([], { status: 500 });
  }
}
