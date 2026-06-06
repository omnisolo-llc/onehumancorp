import { NextResponse } from 'next/server';

export async function GET() {
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:18789';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/audience`, {
      method: 'GET',
      headers: { 'Content-Type': 'application/json' },
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        return NextResponse.json({ error: "Failed to fetch from backend" }, { status: 500 });
    }
  } catch (error) {
    console.error("Error fetching audience sizes:", error);
    return NextResponse.json({ error: "Internal error" }, { status: 500 });
  }
}
