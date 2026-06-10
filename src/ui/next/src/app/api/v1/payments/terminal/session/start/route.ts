import { NextResponse } from "next/server";
import { backendHeaders } from "../../../../../ui/backendProxy";

export async function POST(request: Request) {
  const body = await request.json();
  const backendUrl = process.env.OHC_API_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${backendUrl}/api/v1/payments/terminal/session/start`, {
      method: "POST",
      headers: backendHeaders(request),
      body: JSON.stringify(body),
    });

    const data = await res.json();
    return NextResponse.json(data, { status: res.status });
  } catch (err: any) {
    return NextResponse.json({ success: false, error_message: "Backend connection failed" }, { status: 500 });
  }
}
