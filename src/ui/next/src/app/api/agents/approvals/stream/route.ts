import { NextResponse, NextRequest } from "next/server";
import { backendHeaders } from "../../../ui/backendProxy";

export async function GET(req: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";
  const { search } = new URL(req.url);

  try {
    const res = await fetch(`${backendUrl}/api/agents/approvals/stream${search}`, {
      method: "GET",
      headers: backendHeaders(req),
    });

    return new NextResponse(res.body, {
      status: res.status,
      headers: {
        'Content-Type': 'text/event-stream',
        'Cache-Control': 'no-cache',
        'Connection': 'keep-alive',
      },
    });
  } catch {
    return NextResponse.json({ error: "Backend connection failed" }, { status: 500 });
  }
}
