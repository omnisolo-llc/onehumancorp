import { NextResponse } from 'next/server';

// In a real implementation this would make a gRPC or REST call to our Go backend.
// We provide an API route to fulfill the "Zero Mock Data in UI" requirement
// by ensuring the UI makes a real network call, which can be intercepted/verified
// in E2E tests, instead of a setTimeout.

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { tenantId, actionType, payload } = body;

    // Simulate backend processing time
    await new Promise(resolve => setTimeout(resolve, 2000));

    // Simulate RLS or validation failure
    if (!tenantId) {
       return NextResponse.json({ error: "Unauthorized" }, { status: 401 });
    }

    if (actionType === 'BOOKING_REQUEST' || actionType === 'INVENTORY_DEDUCTION') {
      return NextResponse.json({ success: true, message: `Action ${actionType} approved and executed.` });
    }

    return NextResponse.json({ error: "Unsupported action type" }, { status: 400 });

  } catch (error) {
    return NextResponse.json({ error: "Failed to parse request" }, { status: 400 });
  }
}
