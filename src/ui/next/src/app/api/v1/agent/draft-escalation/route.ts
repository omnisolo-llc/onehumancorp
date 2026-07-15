import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    // Simulate LLM draft
    const draft = `Regarding: ${body.context}\n\nPlease review this shift summary. It requires owner attention.`;
    return NextResponse.json({ draft });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to draft escalation' }, { status: 500 });
  }
}
