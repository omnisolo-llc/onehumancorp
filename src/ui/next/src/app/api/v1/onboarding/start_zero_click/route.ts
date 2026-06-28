import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const payload = await request.json();

    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    let backendRes;

    try {
        backendRes = await fetch(`${backendUrl}/api/onboarding/start_zero_click`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload),
        });
    } catch (e) {
        return NextResponse.json({ error: "Backend communication failed" }, { status: 502 });
    }

    if (backendRes && backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        return NextResponse.json({ error: "Failed to generate response" }, { status: 500 });
    }
  } catch (error) {
    console.error("Error in start_zero_click:", error);
    return NextResponse.json(
      { error: "Internal Server Error" },
      { status: 500 }
    );
  }
}
