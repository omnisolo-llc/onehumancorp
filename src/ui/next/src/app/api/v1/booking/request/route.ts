import { NextResponse } from "next/server";

export async function POST(req: Request) {
  try {
    const body = await req.json();
    const { description, fileName, timestamp, tenant_id } = body;

    const backendUrl = process.env.BACKEND_URL || "http://localhost:8080";

    const res = await fetch(`${backendUrl}/api/agents/webhook`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        tenant_id: tenant_id || "DEFAULT",
        message: description,
        source: "internal_booking",
      }),
    });

    if (!res.ok) {
      return NextResponse.json(
        { error: "Failed to dispatch event" },
        { status: 500 },
      );
    }

    return NextResponse.json({
      success: true,
      request_id: "mock_req_" + Date.now(),
      status: "pending_agent_review",
    });
  } catch (e: any) {
    return NextResponse.json({ error: e.message }, { status: 500 });
  }
}
