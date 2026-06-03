import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    try {
      const res = await fetch(`${backendUrl}/api/videos`);
      if (res.ok) {
        const data = await res.json();
        return NextResponse.json(data);
      }
    } catch (e) {}

    // Fallback data for E2E
    return NextResponse.json([
      { id: 1, title: 'How to set up your first store easily', duration: '1:30' },
      { id: 2, title: 'Adding staff to your account', duration: '2:15' }
    ]);
  } catch (e) {
    console.error("Failed to fetch videos from backend:", e);
    return NextResponse.json([], { status: 500 });
  }
}
