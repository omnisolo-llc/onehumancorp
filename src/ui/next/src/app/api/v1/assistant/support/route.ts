import { NextResponse } from 'next/server';
import { createSupportTicket } from '../store';

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json({ ticket: createSupportTicket(payload || {}) }, { status: 201 });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'support ticket could not be created' }, { status: 400 });
  }
}
