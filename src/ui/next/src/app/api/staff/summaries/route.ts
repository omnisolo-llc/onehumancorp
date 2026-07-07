import { NextResponse } from 'next/server';

export async function GET() {
  try {
    const backendUrl = process.env.API_BASE_URL || 'http://localhost:8080';
    const response = await fetch(`${backendUrl}/api/staff/summaries`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
        'x-spiffe-id': 'spiffe://ohc/org/test_tenant/agent/test_agent', // Mocked identity for local testing
        'x-tenant-id': 'e2e-tenant'
      }
    });

    if (response.ok) {
      const data = await response.json();
      return NextResponse.json(data);
    } else {
      return NextResponse.json({ error: 'Failed to fetch summaries', summaries: [] }, { status: response.status });
    }
  } catch (error) {
    console.error("Error fetching staff summaries:", error);
    return NextResponse.json({ error: 'Internal Server Error', summaries: [] }, { status: 500 });
  }
}
