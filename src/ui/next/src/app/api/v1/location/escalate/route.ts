import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    return NextResponse.json({ success: true, escalated: body.draft });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to escalate' }, { status: 500 });
  }
}
