import { NextResponse, NextRequest } from "next/server";

export const dynamic = "force-dynamic";

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";

  try {
    const res = await fetch(`${backendUrl}/api/api-docs-spec`).catch(
      () => null,
    );

    if (res && res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({ error: "Backend unavailable" }, { status: 502 });
  } catch (e) {
    if (process.env.NODE_ENV !== "test")
      console.error("Failed to fetch api-docs-spec from backend:", e);
    return NextResponse.json({ error: "Backend unavailable" }, { status: 502 });
  }
}
