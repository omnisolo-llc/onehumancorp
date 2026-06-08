import { NextResponse } from "next/server";
import { backendHeaders } from "../../ui/backendProxy";

function clientSecret(payload: any): string | undefined {
  return payload?.client_secret || payload?.intent_id || payload?.Ok?.client_secret || payload?.Ok?.intent_id;
}

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";

  try {
    const body = await req.json();
    const amount = Number(body.amount ?? body.amount_cents);
    const currency = String(body.currency || "usd").toLowerCase();
    const res = await fetch(`${backendUrl}/api/v1/payments/terminal/intent`, {
      method: "POST",
      headers: backendHeaders(req, true),
      body: JSON.stringify({ amount_cents: amount, currency }),
    });
    const data = await res.json();
    if (!res.ok || data?.Err) {
      return NextResponse.json({ error: data?.Err || "Failed to create Terminal payment intent" }, { status: res.status });
    }

    const secret = clientSecret(data);
    if (!secret) {
      return NextResponse.json({ error: "Backend response did not include a PaymentIntent client secret" }, { status: 502 });
    }

    return NextResponse.json({ client_secret: secret });
  } catch {
    return NextResponse.json({ error: "Backend connection failed" }, { status: 500 });
  }
}
