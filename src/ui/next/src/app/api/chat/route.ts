import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const backendUrl = process.env.OHC_CORE_URL || 'http://127.0.0.1:18789';
  const { message } = await req.json();

  try {
    const res = await fetch(`${backendUrl}/api/chat`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ message }),
    });

    if (!res.ok) {
      throw new Error(`Backend API returned status ${res.status}`);
    }

    const data = await res.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error('Error forwarding to backend /api/chat:', error);
    return NextResponse.json(
      { reply: "Sorry, I'm having trouble connecting right now." },
      { status: 500 }
    );
  }
}
