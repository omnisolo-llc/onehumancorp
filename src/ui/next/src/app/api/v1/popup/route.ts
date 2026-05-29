import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const data = await req.json();
    return NextResponse.json({ success: true, message: 'Popup initialized', data });
  } catch (e) {
    return NextResponse.json({ success: false, error: 'Failed to initialize popup' }, { status: 500 });
  }
}
