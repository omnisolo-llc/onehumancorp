import { NextRequest, NextResponse } from "next/server";

export async function POST(req: NextRequest) {
  try {
    const backendUrl = process.env.BACKEND_API_URL || "http://127.0.0.1:18789";
    const body = await req.json();
    const { search } = new URL(req.url);

    const res = await fetch(`${backendUrl}/api/triage/action${search}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        ...(req.headers.get("authorization") ? { authorization: req.headers.get("authorization") as string } : {}),
      },
      body: JSON.stringify(body),
    });

    if (!res.ok) {
      return NextResponse.json({ error: "Backend error" }, { status: res.status });
    }

    const data = await res.json();
    return NextResponse.json(data);
  } catch (e: any) {
    return NextResponse.json({ error: e.message }, { status: 500 });
  }
}