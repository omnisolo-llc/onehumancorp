import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const tenantId = request.headers.get('x-tenant-id') || 'storefront';
    const headers: Record<string, string> = {
      'x-tenant-id': tenantId,
      'Content-Type': 'application/json',
    };
    const authHeader = request.headers.get('Authorization');
    if (authHeader) {
      headers['Authorization'] = authHeader;
    }

    const res = await fetch(`${backendUrl}/api/v1/field-ops/appointments/delay`, {
      method: 'POST',
      headers,
      body: JSON.stringify({
        appointment_id: payload?.appointmentId,
        delay_minutes: payload?.delayMinutes
      }),
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json({ success: true, ...data }, { status: 200 });
    } else {
      return NextResponse.json({ error: 'Failed to process delay' }, { status: res.status });
    }
  } catch (error) {
    console.error('Failed to update delay in backend:', error);
    return NextResponse.json({ error: 'Backend unavailable' }, { status: 502 });
  }
}
