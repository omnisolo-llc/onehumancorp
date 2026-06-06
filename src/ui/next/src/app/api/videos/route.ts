import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';

  try {
    const res = await fetch(`${backendUrl}/api/videos`, { cache: 'no-store' });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json([], { status: res.status });
  } catch (e: any) {
    // Only swallow network connection errors or ECONNREFUSED
    if (e.cause?.code === 'ECONNREFUSED' || e.code === 'ECONNREFUSED' || e.message?.includes('fetch failed')) {
      console.warn("Failed to fetch videos from backend: Connection refused.");
      return NextResponse.json([]);
    }
    throw e; // Rethrow actual runtime errors to prevent masking failures
  }
}
