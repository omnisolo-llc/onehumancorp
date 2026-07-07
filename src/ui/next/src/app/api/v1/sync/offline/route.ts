import { NextResponse } from 'next/server';
import { backendHeaders } from "../../../ui/backendProxy";

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

    const headers = backendHeaders(request, true);
    if (!headers.has("x-spiffe-id") && request.headers.has("x-spiffe-id")) {
       headers.set("x-spiffe-id", request.headers.get("x-spiffe-id") as string);
    }

    const res = await fetch(`${backendUrl}/api/v1/sync/pos_queue`, {
      method: 'POST',
      headers,
      body: JSON.stringify(body)
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data, { status: 200 });
    } else {
      const errorText = await res.text();
      return NextResponse.json({ success: false, error: errorText }, { status: res.status });
    }
  } catch (err: any) {
    console.error("Failed to forward offline pos transactions:", err);
    return NextResponse.json({ success: false, error: err.message }, { status: 500 });
  }
}
