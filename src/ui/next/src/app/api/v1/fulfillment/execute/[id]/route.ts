import { NextResponse } from 'next/server';

export async function POST(request: Request, { params }: { params: { id: string } }) {
  const body = await request.json();

  if (body.action === 'request_driver') {
    return NextResponse.json({
      success: true,
      tracking_url: 'https://doordash.com/tracking/dd_del_123',
      status: 'DriverRequested'
    });
  }

  return NextResponse.json({ success: false, error: 'Unknown action' });
}
