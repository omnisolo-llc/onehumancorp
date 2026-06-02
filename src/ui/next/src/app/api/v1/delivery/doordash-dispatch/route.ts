import { NextResponse, NextRequest } from 'next/server';

export async function POST(req: NextRequest) {
  // Simulate dispatching a Dasher via DoorDash Drive API
  const body = await req.json();
  const orderId = body.orderId || 'unknown';

  await new Promise(resolve => setTimeout(resolve, 1000));

  return NextResponse.json({
    success: true,
    trackingUrl: `https://doordash.com/track/mock_${orderId}`,
    dasherStatus: 'en_route_to_pickup',
    message: "Dasher successfully dispatched"
  }, { status: 200 });
}
