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

  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${backendUrl}/api/chat`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ message: message.trim() })
    });

    if (!res.ok) {
      throw new Error(`Backend responded with status: ${res.status}`);
    }

    const data = await res.json();

    return NextResponse.json({
      reply: data.reply || "I've received your request.",
      link: data.link
    });
  } catch (error) {
    console.error("Failed to connect to backend AI agent:", error);
    return NextResponse.json({
      reply: "I'm having trouble connecting to my brain right now. Please try again later."
    });
  }
}
