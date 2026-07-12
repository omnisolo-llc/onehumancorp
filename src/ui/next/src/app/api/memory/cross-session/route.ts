import { NextResponse } from 'next/server';

const BACKEND_URL = process.env.API_URL || 'http://localhost:8081';

export async function POST(request: Request) {
  try {
    const authHeader = request.headers.get("Authorization");
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
    };
    if (authHeader) headers["Authorization"] = authHeader;

    const body = await request.json();

    const res = await fetch(`${BACKEND_URL}/api/memory/cross-session-search`, {
      method: 'POST',
      headers,
      body: JSON.stringify(body),
    });

    if (!res.ok) {
       console.error("Backend POST failed:", res.status, res.statusText);
       return NextResponse.json({ error: "Backend error", status: res.statusText }, { status: res.status });
    }

    const data = await res.json();
    return NextResponse.json(data);
  } catch (e: any) {
    return NextResponse.json({ error: e.message }, { status: 500 });
  }
}
