import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  // During static export / build time, mock the backend call.
  if (process.env.NODE_ENV === 'production' && !process.env.BACKEND_URL) {
      return NextResponse.json([
          { id: 1, title: "How to set up your first store easily", duration: "1:20" },
          { id: 2, title: "Accept your first payment", duration: "1:15" },
      ]);
  }

  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';

  try {
    const res = await fetch(`${backendUrl}/api/videos`);

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json([], { status: res.status });
  } catch (e) {
    if (process.env.NODE_ENV !== "test") console.error("Failed to fetch videos from backend:", e);
    return NextResponse.json([], { status: 500 });
  }
}
