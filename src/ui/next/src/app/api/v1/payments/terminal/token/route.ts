import { NextResponse } from 'next/server';

export async function POST() {
  return NextResponse.json({ secret: 'mock_terminal_connection_token' });
}
