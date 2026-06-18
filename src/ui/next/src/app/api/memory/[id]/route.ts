import { NextResponse } from 'next/server';

const BACKEND_URL = process.env.API_URL || 'http://localhost:8081';

export async function DELETE(request: Request, { params }: { params: Promise<{ id: string }> }) {
  try {
    const authHeader = request.headers.get("Authorization");
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
    };
    if (authHeader) headers["Authorization"] = authHeader;

    const res = await fetch(`${BACKEND_URL}/api/memory/${(await params).id}`, {
      method: 'DELETE',
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
