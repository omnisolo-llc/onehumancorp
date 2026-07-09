import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
  try {
    const backendUrl = process.env.NEXT_PUBLIC_API_URL || 'http://127.0.0.1:18789';
    const headers = new Headers();
    const authHeader = request.headers.get('authorization');
    if (authHeader) headers.set('authorization', authHeader);
    const cookie = request.headers.get('cookie');
    if (cookie) headers.set('cookie', cookie);

    const { searchParams } = new URL(request.url);
    const tenantId = searchParams.get('tenant_id') || 'default';

    try {
      const backendRes = await fetch(`${backendUrl}/api/v1/growth/upgrade-paywall?tenant_id=${encodeURIComponent(tenantId)}`, {
        method: 'GET',
        headers,
      });

      if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
      }
    } catch (e) {
      if (process.env.NODE_ENV !== "test") {
        console.warn("Backend unavailable, failing upgrade paywall request:", e);
      }
    }

    return NextResponse.json({ error: 'Failed to fetch upgrade paywall status' }, { status: 502 });
  } catch (error) {
    if (process.env.NODE_ENV !== "test") console.error("Error fetching upgrade paywall status:", error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
