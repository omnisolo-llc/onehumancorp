import { NextResponse } from 'next/server';

const BACKEND_URL = process.env.API_BASE_URL || 'http://localhost:8080';

export async function GET(request: Request) {
  try {
    const res = await fetch(`${BACKEND_URL}/api/onboarding/state`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
        // Pass relevant auth headers if needed
        'X-Tenant-ID': request.headers.get('X-Tenant-ID') || 'default_tenant',
        'X-User-ID': request.headers.get('X-User-ID') || 'default_user'
      }
    });

    if (!res.ok) {
      return NextResponse.json({ error: 'Backend error' }, { status: res.status });
    }

    const data = await res.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error("Error proxying GET /api/onboarding/state", error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}

export async function POST(request: Request) {
  try {
    const body = await request.json();

    const res = await fetch(`${BACKEND_URL}/api/onboarding/state`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'X-Tenant-ID': request.headers.get('X-Tenant-ID') || 'default_tenant',
        'X-User-ID': request.headers.get('X-User-ID') || 'default_user'
      },
      body: JSON.stringify(body)
    });

    if (!res.ok) {
      return NextResponse.json({ error: 'Backend error' }, { status: res.status });
    }

    return NextResponse.json({ success: true });
  } catch (error) {
    console.error("Error proxying POST /api/onboarding/state", error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
