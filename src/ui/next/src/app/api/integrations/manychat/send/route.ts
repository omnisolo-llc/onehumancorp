import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { subscriber_id, message } = await req.json();

    // In a real application, we would call the ManyChat API to send a message.
    // For now, we simulate a successful API response.
<<<<<<< HEAD
=======
    console.log(`Sending message to subscriber ${subscriber_id}: ${message}`);
>>>>>>> 8ae292f8 (docs: ohc market dynamics and deep-dive competitor analysis (#22647))

    return NextResponse.json({ success: true });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to send message' }, { status: 500 });
  }
}
