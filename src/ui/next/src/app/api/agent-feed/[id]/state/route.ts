import { NextResponse } from "next/server";
import { backendHeaders } from "../../../ui/backendProxy";

export async function PUT(req: Request, context: any) {
  const id = context.params?.id || (await context.params)?.id;
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";
  const { search } = new URL(req.url);

  try {
    const body = await req.json();
    const res = await fetch(`${backendUrl}/api/agent-feed/${id}/state${search}`, {
      method: "PUT",
      headers: backendHeaders(req, true),
      body: JSON.stringify(body),
    });
    return NextResponse.json(await res.json(), { status: res.status });
  } catch {
    return NextResponse.json({ error: "Backend connection failed" }, { status: 500 });
  }
}
