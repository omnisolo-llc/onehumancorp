import { NextResponse } from 'next/server';
import { cookies } from 'next/headers';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const backendUrl = process.env.OHC_API_URL || 'http://localhost:8080';

    // In production, extract the authenticated tenant context from a secure JWT session or cookies
    const cookieStore = cookies();
    const sessionCookie = cookieStore.get('ohc_session');

    // If no valid secure session exists, we don't proceed with elevated actions
    if (!sessionCookie && process.env.NODE_ENV === 'production') {
        return NextResponse.json({ success: false, error: 'Unauthorized' }, { status: 401 });
    }

    // Pass the standard auth cookie or securely parsed identity
    // Here we assume the backend validates the session cookie instead of trusting a header

    const res = await fetch(`${backendUrl}/api/v1/payments/terminal/inventory/lock`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Cookie': `ohc_session=${sessionCookie?.value || ''}`
      },
      body: JSON.stringify(body)
    });

    if (res.ok) {
        const data = await res.json();
        return NextResponse.json(data);
    }

    return NextResponse.json({ success: false }, { status: res.status });
  } catch (error) {
    return NextResponse.json({ success: false }, { status: 500 });
  }
}
