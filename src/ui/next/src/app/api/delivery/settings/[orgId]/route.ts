import { NextResponse } from 'next/server';

// Mock storage for demo purposes
let deliverySettings: any = {
  enabled: false,
  radius_miles: 5.0,
  flat_fee_cents: 850,
};

export async function GET(request: Request, { params }: { params: { orgId: string } }) {
  return NextResponse.json(deliverySettings);
}

export async function POST(request: Request, { params }: { params: { orgId: string } }) {
  const body = await request.json();
  deliverySettings = { ...deliverySettings, ...body };
  return NextResponse.json(deliverySettings);
}
