import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const backendUrl = process.env.API_BASE_URL || process.env.BACKEND_URL || 'http://localhost:8080';

    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
    };

    const spiffeId = request.headers.get('x-spiffe-id');
    if (spiffeId) headers['x-spiffe-id'] = spiffeId;

    const tenantId = request.headers.get('x-tenant-id');
    if (tenantId) headers['x-tenant-id'] = tenantId;

    const authHeader = request.headers.get('Authorization');
    if (authHeader) headers['Authorization'] = authHeader;

    const response = await fetch(`${backendUrl}/api/staff/escalate`, {
      method: 'POST',
      headers,
      body: JSON.stringify(body)
    });

    if (response.ok) {
      const data = await response.json();
      return NextResponse.json(data);
    } else {
      return NextResponse.json({ error: 'Failed to escalate issue' }, { status: response.status });
    }
  } catch (error) {
    console.error("Error escalating issue:", error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
