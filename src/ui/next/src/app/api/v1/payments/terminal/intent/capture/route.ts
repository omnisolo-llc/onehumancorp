import { NextResponse } from "next/server";
import { backendHeaders } from "../../../../../ui/backendProxy";

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";

  try {
    const body = await req.json();

    const res = await fetch(`${backendUrl}/api/v1/payments/terminal/intent/capture`, {
      method: "POST",
      headers: backendHeaders(req),
      body: JSON.stringify(body),
    });

    if (res.ok) {
        return NextResponse.json(await res.json(), { status: res.status });
    }

    return NextResponse.json({ success: false, error_message: await res.text() }, { status: res.status });
  } catch (err: any) {
    return NextResponse.json({ error: "Backend connection failed: " + err.message }, { status: 500 });
  }
}
