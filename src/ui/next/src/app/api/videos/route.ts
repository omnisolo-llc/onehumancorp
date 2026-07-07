import { NextResponse, NextRequest } from "next/server";

export const dynamic = 'force-dynamic';

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";

  try {
    const { searchParams } = new URL(request.url);
    const mobileOptimized = searchParams.get("mobile_optimized");

    let fetchUrl = `${backendUrl}/api/videos`;
    if (mobileOptimized) {
      fetchUrl += `?mobile_optimized=${mobileOptimized}`;
    }

    const res = await fetch(fetchUrl).catch(() => null);

    if (res && res.ok) {
      const data = await res.json();
      if (data && data.length > 0) {
        return NextResponse.json(data);
      }
    }

    return NextResponse.json({ error: "Backend unavailable" }, { status: 502 });
  } catch (e) {
    if (process.env.NODE_ENV !== "test" && process.env.CI !== "1") {
      console.error("Failed to fetch videos from backend:", e);
    }
    return NextResponse.json({ error: "Backend unavailable" }, { status: 502 });
  }
}
