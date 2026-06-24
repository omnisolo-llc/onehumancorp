import { NextResponse } from 'next/server';
import { backendHeaders } from "../../../../../ui/backendProxy";

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

    const headers = backendHeaders(request, true);

    const res = await fetch(`${backendUrl}/api/v1/sync/events`, {
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
    console.error("Failed to forward sync events:", err);
    return NextResponse.json({ success: false, error: err.message }, { status: 500 });
  }
}
