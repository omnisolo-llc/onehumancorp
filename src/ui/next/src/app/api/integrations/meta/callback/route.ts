import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.redirect(new URL('/inbox?meta_connected=true', process.env.NEXT_PUBLIC_BASE_URL || 'http://localhost:3000'));
}
