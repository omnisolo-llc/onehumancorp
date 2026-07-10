import { NextResponse, NextRequest } from "next/server";

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";

  try {
    const res = await fetch(`${backendUrl}/api/help`).catch(() => null);

    if (res && res.ok) {
      const data = await res.json();
      if (data && data.length > 0) {
        return NextResponse.json(data);
      }
    }

    return NextResponse.json({ error: "Failed to fetch help" }, { status: 500 });
  } catch (e) {
    if (process.env.NODE_ENV !== "test" && process.env.CI !== "1") {
      console.error("Failed to fetch help from backend:", e);
    }
    return NextResponse.json({ error: "Failed to fetch help" }, { status: 500 });
  }
}
