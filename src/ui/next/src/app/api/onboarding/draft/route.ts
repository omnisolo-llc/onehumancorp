import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  try {
    const res = await fetch(`${backendUrl}/api/onboarding/draft`, {
      headers: {
        'x-tenant-id': tenantId,
        'x-user-id': userId
      }
    });

    if (res.ok) {
      const text = await res.text();
      let data = {};
      if (text) {
        try {
          data = JSON.parse(text);
        } catch (e) {
          console.error("Failed to parse draft GET JSON:", e);
        }
      }
      return NextResponse.json(data);
    }

    return NextResponse.json({});
  } catch (e) {
    return NextResponse.json({});
  }
}

export async function POST(request: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  try {
    const body = await request.json();
    const res = await fetch(`${backendUrl}/api/onboarding/draft`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-tenant-id': tenantId,
        'x-user-id': userId
      },
      body: JSON.stringify(body)
    });

    if (res.ok) {
      const text = await res.text();
      let data = {};
      if (text) {
         try {
           data = JSON.parse(text);
         } catch (e) {
           console.error("Failed to parse draft POST JSON:", e);
         }
      }
      return NextResponse.json(data);
    }

    return new NextResponse(null, { status: 200 });
  } catch (e) {
    return new NextResponse(null, { status: 200 });
  }
}
