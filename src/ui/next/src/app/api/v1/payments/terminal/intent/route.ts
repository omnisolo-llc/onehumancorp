import { NextResponse } from "next/server";
import { backendHeaders } from "../../../../ui/backendProxy";

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";

  try {
    const body = await req.json();

    // Normalize body to Rust API expectations
    const payload = {
      amount_cents: body.amount_cents || body.amount,
      currency: body.currency || "usd",
      product_id: body.product_id || null,
      quantity: body.quantity || null,
      order_id: body.order_id || null,
    };

    const res = await fetch(`${backendUrl}/api/v1/payments/terminal/intent`, {
      method: "POST",
      headers: backendHeaders(req),
      body: JSON.stringify(payload),
    });
    const data = await res.json();

    if (!res.ok || data?.Err) {
      return NextResponse.json({ error: data?.Err || "Failed to create PaymentIntent" }, { status: res.status });
    }

    return NextResponse.json(data.Ok || data);
  } catch (err: any) {
    return NextResponse.json({ error: "Backend connection failed: " + err.message }, { status: 500 });
  }
}
