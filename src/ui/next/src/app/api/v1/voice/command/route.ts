import { NextResponse } from "next/server";
import { backendHeaders } from "../../../ui/backendProxy";

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";

  try {
    const formData = await req.formData();
    const audio = formData.get("audio");
    if (!audio) {
      return NextResponse.json({ error: "audio is required" }, { status: 400 });
    }

    const tenantId = formData.get("tenant_id") || "default";

    // Convert audio File/Blob to base64
    const buffer = Buffer.from(await (audio as Blob).arrayBuffer());
    const base64Audio = buffer.toString("base64");

    const headers = backendHeaders(req, true);
    headers.set("x-tenant-id", tenantId.toString());

    const res = await fetch(`${backendUrl}/api/v1/voice/command`, {
      method: "POST",
      headers,
      body: JSON.stringify({
        audio_data: base64Audio,
      }),
    });

    if (res.ok) {
      return NextResponse.json(await res.json());
    }

    return NextResponse.json({ error: "Backend request failed" }, { status: res.status });
  } catch (error) {
    console.error(error);
    return NextResponse.json({ error: "Backend connection failed" }, { status: 500 });
  }
}
