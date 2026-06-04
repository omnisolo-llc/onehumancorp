import { NextResponse } from 'next/server';

const MAX_MESSAGE_LENGTH = 1000;

export async function POST(req: Request) {
  let body: unknown;
  try {
    body = await req.json();
  } catch {
    return NextResponse.json({ error: "Invalid JSON body" }, { status: 400 });
  }

  const message = body && typeof body === 'object' && 'message' in body
    ? (body as { message?: unknown }).message
    : undefined;

  if (typeof message !== 'string' || !message.trim()) {
    return NextResponse.json({ error: "message is required" }, { status: 400 });
  }

  if (message.trim().length > MAX_MESSAGE_LENGTH) {
    return NextResponse.json({ error: `message must be ${MAX_MESSAGE_LENGTH} characters or fewer` }, { status: 413 });
  }

  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';

  try {
    const res = await fetch(`${backendUrl}/api/chat`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ message: message.trim() })
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({ error: "Backend error" }, { status: res.status });
  } catch (e) {
    console.error("Failed to fetch chat from backend:", e);
    return NextResponse.json({ error: "Backend communication failed" }, { status: 500 });
  }
}
