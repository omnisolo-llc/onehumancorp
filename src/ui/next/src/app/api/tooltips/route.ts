import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';

  try {
    const res = await fetch(`${backendUrl}/api/tooltips`);

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({}, { status: res.status });
  } catch (e) {
    console.error("Failed to fetch tooltips from backend:", e);
    return NextResponse.json({}, { status: 500 });
  }
}
