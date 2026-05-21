import { NextResponse } from "next/server";

export const dynamic = "force-dynamic";

import { headers } from "next/headers";

export async function POST(
  request: Request,
  { params }: { params: { id: string } }
) {
  try {
    const body = await request.json();
    const backendUrl = process.env.BACKEND_API_URL || 'http://127.0.0.1:8080';
    const res = await fetch(`${backendUrl}/api/agents/approvals/${params.id}`, {
      method: "POST",
      headers: headers(),
      body: JSON.stringify(body)
    });

    if (!res.ok) {
        return NextResponse.json({ error: "Upstream error" }, { status: res.status });
    }
    const data = await res.json();
    return NextResponse.json(data);
  } catch (error) {
    return NextResponse.json({ error: "Invalid request" }, { status: 400 });
  }
}
