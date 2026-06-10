import { NextResponse } from "next/server";
import { backendHeaders } from "../../../backendProxy";

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";
  const { search } = new URL(req.url);

  try {
    const body = await req.json();
    const res = await fetch(`${backendUrl}/api/ui/triage/action${search}`, {
      method: "POST",
      headers: backendHeaders(req, true),
      body: JSON.stringify(body),
    });
    return NextResponse.json(await res.json(), { status: res.status });
  } catch (err) {
    return NextResponse.json({ error: "Backend connection failed" }, { status: 500 });
  }
}
