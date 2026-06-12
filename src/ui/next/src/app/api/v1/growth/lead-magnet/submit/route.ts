import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();

    // In a real application, we would call the Rust backend:
    const backendUrl = process.env.OHC_BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/lead-magnet/submit`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(body),
    });

    if (backendRes.ok) {
      const data = await backendRes.json();
      return NextResponse.json(data);
    } else {
      return NextResponse.json({ error: 'Failed to submit' }, { status: backendRes.status });
    }
  } catch (error) {
    console.error("Error submitting lead magnet form:", error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
