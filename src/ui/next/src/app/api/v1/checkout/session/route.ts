import { NextResponse } from 'next/server';
import { backendHeaders } from "../../../../api/ui/backendProxy";

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  try {
    const body = await req.json();
    const res = await fetch(`${backendUrl}/api/v1/checkout/session`, {
      method: 'POST',
      headers: backendHeaders(req, true),
      body: JSON.stringify(body),
    });

    if (res.ok) {
      return NextResponse.json(await res.json());
    }

    return NextResponse.json({ success: false, error_message: await res.text() }, { status: res.status });
  } catch (e: any) {
    return NextResponse.json({ success: false, error_message: 'Backend connection failed' }, { status: 500 });
  }
}
