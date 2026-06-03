import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  const body = await request.json();

  // Mock logic: available if address is not empty
  if (body.address && body.address.length > 5) {
    return NextResponse.json({
      available: true,
      fee_cents: 850,
      estimated_minutes: 35,
    });
  }

  return NextResponse.json({
    available: false,
    error: "Address not supported or outside delivery radius",
  });
}
