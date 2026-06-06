import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  const body = await request.json();
  const { amount_cents, currency } = body;

  return NextResponse.json({
    client_secret: `mock_pi_secret_${Date.now()}`,
    amount_cents,
    currency
  });
}
