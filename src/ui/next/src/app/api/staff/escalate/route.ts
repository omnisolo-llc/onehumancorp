import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const backendUrl = process.env.API_BASE_URL || 'http://localhost:8080';

    const response = await fetch(`${backendUrl}/api/staff/escalate`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-spiffe-id': 'spiffe://ohc/org/test_tenant/agent/test_agent', // Mocked identity for local testing
        'x-tenant-id': 'e2e-tenant'
      },
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
