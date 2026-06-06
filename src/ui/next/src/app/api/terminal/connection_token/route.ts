import { NextResponse } from 'next/server';

export async function POST() {
  const backendUrl = process.env.OHC_API_URL || 'http://localhost:8080';
  const res = await fetch(`${backendUrl}/api/v1/payments/terminal/token`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
  });

  if (!res.ok) {
      return NextResponse.json({error: "Failed to get token"}, { status: 500 });
  }

  const data = await res.json();
  return NextResponse.json({ secret: data.token });
}
