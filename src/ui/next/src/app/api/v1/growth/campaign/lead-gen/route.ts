import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  return NextResponse.json({ success: true, message: 'Campaign started' }, { status: 200 });
}
