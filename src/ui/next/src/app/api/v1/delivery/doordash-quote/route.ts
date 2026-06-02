import { NextResponse } from 'next/server';

export async function GET() {
  // Simulate fetching a quote from DoorDash Drive API
  // In a real app this would query the OHC Backend which would hit DoorDash

  // Return a flat fee. We mock a short delay to simulate network latency
  await new Promise(resolve => setTimeout(resolve, 800));

  return NextResponse.json({
    fee: 7.50,
    estimated_time: "35 mins",
    currency: "USD"
  }, { status: 200 });
}
