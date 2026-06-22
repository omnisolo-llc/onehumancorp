import { NextResponse, NextRequest } from "next/server";

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";

  try {
    const res = await fetch(`${backendUrl}/api/tooltips`).catch(() => null);

    if (res && res.ok) {
      const data = await res.json();
      if (data && Object.keys(data).length > 0) {
        return NextResponse.json(data);
      }
    }

    return NextResponse.json({}, { status: 200 });
  } catch (e) {
    if (process.env.NODE_ENV !== "test")
      console.error("Failed to fetch tooltips from backend:", e);
    return NextResponse.json({}, { status: 200 });
  }
}

export async function POST(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";

  try {
    const body = await request.json();
    const res = await fetch(`${backendUrl}/api/tooltips`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body)
    }).catch(() => null);

    if (res && res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({ success: false }, { status: 500 });
  } catch (e) {
    if (process.env.NODE_ENV !== "test")
      console.error("Failed to update tooltip:", e);
    return NextResponse.json({ success: false }, { status: 500 });
  }
}
