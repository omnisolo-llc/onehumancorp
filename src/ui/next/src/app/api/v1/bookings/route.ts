import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json({
    bookings: [
      { id: "1", customerName: "Alice Smith", serviceName: "Plumbing Fix", startTime: new Date(Date.now() + 86400000).toISOString(), status: "scheduled" },
      { id: "2", customerName: "Bob Johnson", serviceName: "Painting", startTime: new Date(Date.now() + 172800000).toISOString(), status: "pending" },
    ]
  });
}
