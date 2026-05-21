import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const response = await fetch('http://127.0.0.1:8080/api/onboarding/start', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(body),
    });

    if (!response.ok) {
        return NextResponse.json({error: "Backend error"}, {status: response.status});
    }

    const data = await response.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error('Error starting onboarding:', error);
    return NextResponse.json({ error: 'Failed to start onboarding' }, { status: 500 });
  }
}
