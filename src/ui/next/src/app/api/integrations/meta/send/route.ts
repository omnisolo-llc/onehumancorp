import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const { message, to } = await req.json();
  console.log(`[Meta API Mock] Sending message to ${to}: ${message}`);
  return NextResponse.json({ success: true, message: 'Message queued for delivery' });
}
