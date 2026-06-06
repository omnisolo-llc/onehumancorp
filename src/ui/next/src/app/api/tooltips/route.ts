import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';

  try {
    const res = await fetch(`${backendUrl}/api/tooltips`);

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({ error: `Backend returned ${res.status}` }, { status: res.status });
  } catch (e: unknown) {
    const message = e instanceof Error ? e.message : 'Unknown error';
    console.error("Failed to fetch tooltips from backend:", message);
    return NextResponse.json({ error: "Failed to fetch tooltips" }, { status: 500 });
  }
}
