import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  const backendUrl = process.env.OHC_API_URL || 'http://localhost:8080';
  const body = await request.json();

  const res = await fetch(`${backendUrl}/api/v1/sync/offline`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body)
  });

  if (!res.ok) {
      return NextResponse.json({error: "Failed to sync offline"}, { status: 500 });
  }

  const data = await res.json();
  return NextResponse.json(data);
}
