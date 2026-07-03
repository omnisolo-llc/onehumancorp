import { NextResponse } from 'next/server';

const BACKEND_URL = process.env.API_URL || 'http://localhost:8081';

export async function GET(request: Request) {
  try {
    const authHeader = request.headers.get("Authorization");
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
    };
    if (authHeader) headers["Authorization"] = authHeader;

    const res = await fetch(`${BACKEND_URL}/api/memory`, {
      headers,
    });

    if (!res.ok) {
       console.error("Backend GET failed:", res.status, res.statusText);
       // Instead of returning mock data, return the real error
       return NextResponse.json({ error: "Backend error", status: res.statusText }, { status: res.status });
    }

    const text = await res.text();
    if (!text) {
        return NextResponse.json([]);
    }

    const data = JSON.parse(text);
    return NextResponse.json(data);
  } catch (e: any) {
    return NextResponse.json({ error: e.message }, { status: 500 });
  }
}
