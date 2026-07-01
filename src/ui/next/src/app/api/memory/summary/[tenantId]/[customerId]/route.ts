import { NextResponse } from 'next/server';

const BACKEND_URL = process.env.API_URL || 'http://localhost:18789';

export async function GET(request: Request, { params }: { params: Promise<{ tenantId: string, customerId: string }> }) {
  try {
    const authHeader = request.headers.get("Authorization");
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
    };
    if (authHeader) headers["Authorization"] = authHeader;

    const { tenantId, customerId } = await params;

    const res = await fetch(`${BACKEND_URL}/api/memory/summary/${tenantId}/${customerId}`, {
      method: 'GET',
      headers,
    });

    if (!res.ok) {
        return NextResponse.json({ error: "Backend error" }, { status: res.status });
    }
    const data = await res.json();
    return NextResponse.json(data);
  } catch (e: any) {
    return NextResponse.json({ error: e.message }, { status: 500 });
  }
}
