import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const body = await req.json();
    const { orderId, weight, dimensions } = body;

    // Simulate fetching rates from Shippo backend
    // In a real app, this would call the Go backend which calls Shippo

    await new Promise(resolve => setTimeout(resolve, 1000)); // Simulate latency

    const mockRates = [
      { id: 'rate_usps_1', carrier: 'USPS', service: 'Priority Mail', amount: '8.50', days: 2 },
      { id: 'rate_usps_2', carrier: 'USPS', service: 'First-Class Mail', amount: '4.20', days: 4 },
      { id: 'rate_ups_1', carrier: 'UPS', service: 'Ground', amount: '9.75', days: 3 },
    ];

    return NextResponse.json({ rates: mockRates });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to fetch rates' }, { status: 500 });
  }
}
