import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const data = await req.json();
    const backendUrl = process.env.OHC_API_URL || "http://backend.internal";

    const res = await fetch(`${backendUrl}/api/v1/booking/reserve_time_slot`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });

    if (res.ok) {
      const responseData = await res.json();
      return NextResponse.json(responseData);
    } else {
      return NextResponse.json({ error: 'Failed to reserve time slot' }, { status: res.status });
    }
  } catch (error) {
    console.error("Error in /api/v1/booking/reserve_time_slot route:", error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
