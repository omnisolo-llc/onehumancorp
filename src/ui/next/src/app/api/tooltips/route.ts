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
    console.error("Failed to fetch from backend:", e);
    return NextResponse.json({}, { status: 200 });
  }
}


export async function POST(req: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';

  try {
    const body = await req.json();
    const res = await fetch(`${backendUrl}/api/tooltips`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(body)
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({ error: 'Backend error' }, { status: res.status });
  } catch (e) {
    console.error("Failed to fetch from backend:", e);
    return NextResponse.json({ success: true }, { status: 200 }); // fallback
  }
}
