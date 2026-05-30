import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    return NextResponse.json({ success: true, processed: body.queue?.length || 0 });
  } catch (e) {
    return NextResponse.json({ success: false, error: 'Failed to process sync' }, { status: 500 });
  }
}
