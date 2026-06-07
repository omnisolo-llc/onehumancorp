import { NextResponse } from "next/server";
import { backendHeaders } from "../../../../ui/backendProxy";

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";
  try {
    const body = await req.json();
    const res = await fetch(`${backendUrl}/api/v1/payments/terminal/session/start`, {
      method: "POST",
      headers: backendHeaders(req),
      body: JSON.stringify(body),
    });
    const data = await res.json();
    if (!res.ok) {
      return NextResponse.json({ error: data?.error || "Failed to start terminal session" }, { status: res.status });
    }
    return NextResponse.json(data);
  } catch (e) {
    return NextResponse.json({ error: "Backend connection failed" }, { status: 500 });
  }
}
