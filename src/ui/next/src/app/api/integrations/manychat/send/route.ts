import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { subscriber_id, message } = await req.json();

    // In a real application, we would call the ManyChat API to send a message.
    // For now, we simulate a successful API response.

    return NextResponse.json({ success: true });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to send message' }, { status: 500 });
  }
}
