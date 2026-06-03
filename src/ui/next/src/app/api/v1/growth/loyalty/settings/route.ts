import { NextResponse } from 'next/server';

const BACKEND_URL = process.env.BACKEND_URL || 'http://localhost:8080';

export async function GET(request: Request) {
  try {
    const tenantId = request.headers.get("x-tenant-id") || 'test-tenant';
    const res = await fetch(`${BACKEND_URL}/api/v1/growth/loyalty/settings`, {
      headers: {
        "x-tenant-id": tenantId
      }
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    } else {
      return NextResponse.json({ error: "Failed to get loyalty settings" }, { status: res.status });
    }
  } catch (error) {
    console.error("Error calling backend:", error);
    return NextResponse.json({ error: "Internal Server Error" }, { status: 500 });
  }
}

export async function POST(request: Request) {
  try {
    const tenantId = request.headers.get("x-tenant-id") || 'test-tenant';
    const body = await request.json();
    const res = await fetch(`${BACKEND_URL}/api/v1/growth/loyalty/settings`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        "x-tenant-id": tenantId
      },
      body: JSON.stringify(body),
    });

    if (res.ok) {
      return NextResponse.json({ success: true });
    } else {
      return NextResponse.json({ error: "Failed to update loyalty settings" }, { status: res.status });
    }
  } catch (error) {
    console.error("Error calling backend:", error);
    return NextResponse.json({ error: "Internal Server Error" }, { status: 500 });
  }
}
