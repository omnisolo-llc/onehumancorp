import { NextResponse } from 'next/server';

const BACKEND_URL = process.env.API_BASE_URL || 'http://localhost:8080';

export async function POST(request: Request) {
  try {
    const body = await request.json();

    const res = await fetch(`${BACKEND_URL}/api/onboarding/start`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(body)
    });

    if (!res.ok) {
      return NextResponse.json({ error: 'Backend error' }, { status: res.status });
    }

    const data = await res.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error("Error proxying POST /api/onboarding/start", error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
