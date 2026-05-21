import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  try {
    const response = await fetch('http://127.0.0.1:8080/api/onboarding/state', {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
        'X-Tenant-ID': 'default_tenant', // Or get from session
      },
    });

    if (!response.ok) {
        return NextResponse.json({error: "Backend error"}, {status: response.status});
    }

    const data = await response.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error('Error getting state:', error);
    return NextResponse.json({ error: 'Failed to get state' }, { status: 500 });
  }
}

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const response = await fetch('http://127.0.0.1:8080/api/onboarding/state', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'X-Tenant-ID': 'default_tenant', // Or get from session
        'X-User-ID': 'default_user', // Or get from session
      },
      body: JSON.stringify(body),
    });

    if (!response.ok) {
        return NextResponse.json({error: "Backend error"}, {status: response.status});
    }

    return new NextResponse(null, { status: 204 });
  } catch (error) {
    console.error('Error saving state:', error);
    return NextResponse.json({ error: 'Failed to save state' }, { status: 500 });
  }
}
