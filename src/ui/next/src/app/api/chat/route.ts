import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  const { message } = await request.json();

  try {
    const baseUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    const response = await fetch(`${baseUrl}/api/v1/ai/draft-reply`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        customer_message: message,
        business_context: "A friendly business helping customers."
      }),
    });

    if (!response.ok) {
      return NextResponse.json({ reply: "Sorry, I'm having trouble connecting right now." }, { status: 500 });
    }

    const data = await response.json();
    return NextResponse.json({ reply: data.output });
  } catch (error) {
    return NextResponse.json({ reply: "Sorry, I'm having trouble connecting right now." }, { status: 500 });
  }
}
