import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();

    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    let backendRes;
    try {
        backendRes = await fetch(`${backendUrl}/api/v1/growth/promoter/generate`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(body),
        });
    } catch (e) { return NextResponse.json({ error: "Network error" }, { status: 502 }); }

    if (backendRes && backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else { return NextResponse.json({ error: "Backend error" }, { status: 502 }); }
  } catch (error) {
    console.error("Error generating promoter posts:", error);
    return NextResponse.json({ error: "Internal server error" }, { status: 500 });
  }
}
