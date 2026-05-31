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

  return NextResponse.json({
    reply: "I am your AI Help Agent! I specialize in answering questions about OHC features and helping you grow your small business. Check out our Getting Started guide.",
    link: { url: "/help", title: "Read the full article →" }
  });
}
