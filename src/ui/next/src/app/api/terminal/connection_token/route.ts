import { NextResponse } from "next/server";
import { backendHeaders } from "../../ui/backendProxy";

function terminalToken(payload: any): string | undefined {
  return payload?.secret || payload?.token || payload?.Ok?.secret || payload?.Ok?.token;
}

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || "http://localhost:8080";

  try {
    const res = await fetch(`${backendUrl}/api/v1/payments/terminal/token`, {
      method: "POST",
      headers: backendHeaders(req),
    });
    const data = await res.json();
    if (!res.ok || data?.Err) {
      return NextResponse.json({ error: data?.Err || "Failed to create Terminal connection token" }, { status: res.status });
    }

    const secret = terminalToken(data);
    if (!secret) {
      return NextResponse.json({ error: "Backend response did not include a Terminal secret" }, { status: 502 });
    }

    return NextResponse.json({ secret });
  } catch {
    return NextResponse.json({ error: "Backend connection failed" }, { status: 500 });
  }
}
