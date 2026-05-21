import { NextResponse } from "next/server";

export const dynamic = "force-dynamic";

export async function POST(
  request: Request,
  { params }: { params: { id: string } }
) {
  try {
    const body = await request.json();

    // Try to proxy to the Rust backend
    const backendUrl = process.env.API_URL || 'http://localhost:18789';
    try {
      const res = await fetch(`${backendUrl}/api/agents/approvals/${params.id}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Cookie': request.headers.get('cookie') || '',
          'Authorization': request.headers.get('authorization') || ''
        },
        body: JSON.stringify(body)
      });
      if (res.ok) {
        const data = await res.json();
        return NextResponse.json(data);
      }
    } catch (e) {
      console.error("Failed to proxy to backend", e);
    }

    // Fallback
    return NextResponse.json({ success: true, id: params.id });
  } catch (error) {
    return NextResponse.json({ error: "Invalid request" }, { status: 400 });
  }
}
