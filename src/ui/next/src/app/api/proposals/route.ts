import { NextResponse } from "next/server";

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";
  const tenantId = req.headers.get("x-tenant-id") || "default";

  try {
    const body = await req.json();
    const res = await fetch(`${backendUrl}/proposals/draft`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "x-tenant-id": tenantId,
      },
      body: JSON.stringify(body),
    });

    if (res.ok) {
      return NextResponse.json(await res.json());
    }

    return NextResponse.json({ error: "Failed to draft proposal" }, { status: res.status });
  } catch {
    return NextResponse.json({ error: "Backend connection failed" }, { status: 500 });
  }
}
