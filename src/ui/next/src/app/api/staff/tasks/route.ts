import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  try {
    const backendUrl = process.env.API_BASE_URL || 'http://localhost:8080';

    // Extract identity headers from incoming request, failing over to defaults
    const authHeader = request.headers.get('Authorization') || request.headers.get('x-spiffe-id') || '';
    const spiffeId = authHeader.includes('spiffe') ? authHeader : 'spiffe://ohc/org/e2e-tenant/agent/browser';

    const tenantId = request.headers.get('x-tenant-id') || 'e2e-tenant';
    const userId = request.headers.get('x-user-id');

    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      'x-spiffe-id': spiffeId,
      'x-tenant-id': tenantId
    };

    if (userId) {
      headers['x-user-id'] = userId;
    }

    const response = await fetch(`${backendUrl}/api/staff/tasks`, {
      method: 'GET',
      headers
    });

    if (response.ok) {
      const data = await response.json();
      return NextResponse.json(data);
    } else {
      return NextResponse.json({ error: 'Failed to fetch tasks', tasks: [] }, { status: response.status });
    }
  } catch (error) {
    console.error("Error fetching staff tasks:", error);
    return NextResponse.json({ error: 'Internal Server Error', tasks: [] }, { status: 500 });
  }
}
