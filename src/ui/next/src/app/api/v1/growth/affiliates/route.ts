import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  try {
    const url = new URL(request.url);
    const tenantId = url.searchParams.get('tenant_id') || 'default-tenant';

    // Proxy request to backend
    const backendUrl = `http://127.0.0.1:18789/api/v1/growth/affiliates?tenant_id=${tenantId}`;
    const response = await fetch(backendUrl, {
      headers: {
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
      if (response.status === 404) {
        // Feature might not be fully wired up in the backend yet, fail gracefully with an empty list
        return NextResponse.json({ affiliates: [] });
      }
      return NextResponse.json(
        { error: 'Failed to fetch affiliates from backend' },
        { status: response.status }
      );
    }

    const data = await response.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error('Error fetching affiliates:', error);
    // On connection error to the proxy, return a truthful empty state instead of breaking the UI
    return NextResponse.json({ affiliates: [] });
  }
}