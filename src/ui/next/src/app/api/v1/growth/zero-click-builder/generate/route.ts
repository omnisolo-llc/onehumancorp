import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { prompt } = await request.json();

    if (!prompt) {
      return NextResponse.json({ error: "Prompt is required" }, { status: 400 });
    }

    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    let backendRes;

    // Instead of using mock static data, we forward the request to the real Rust backend to utilize Minimax LLM
    try {
        backendRes = await fetch(`${backendUrl}/api/v1/growth/zero-click-builder/generate`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ prompt }),
        });
    } catch (e) {
        return NextResponse.json({ error: "Backend communication failed" }, { status: 502 });
    }

    if (backendRes && backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        return NextResponse.json({ error: "Failed to generate store" }, { status: 500 });
    }
  } catch (error) {
    console.error("Error generating zero-click store:", error);
    return NextResponse.json(
      { error: "Internal Server Error" },
      { status: 500 }
    );
  }
}
