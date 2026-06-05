import { NextResponse } from "next/server";

export async function GET(req: Request) {
  try {
    // Stubbing the backend call. In reality we'd use a gRPC client to call BookingEngineService.GetUpcomingBookings.

    return NextResponse.json({
      bookings: [
        {
          booking_id: "mock_123",
          customer_name: "Carlos Handyman",
          service_name: "Plumbing Fix",
          start_time: new Date(Date.now() + 86400000).toISOString(), // Tomorrow
          end_time: new Date(Date.now() + 90000000).toISOString(),
          status: "confirmed",
        },
      ],
    });
  } catch (e: any) {
    return NextResponse.json({ error: e.message }, { status: 500 });
  }
}
