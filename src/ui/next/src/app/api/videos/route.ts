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

    // Fallback static data
    return NextResponse.json([
      { id: 1, title: "How to set up your first store easily", duration: "1:20", video_url: "https://www.w3schools.com/html/mov_bbb.mp4" }
    ]);
  } catch (e) {
    return NextResponse.json([
      { id: 1, title: "How to set up your first store easily", duration: "1:20", video_url: "https://www.w3schools.com/html/mov_bbb.mp4" }
    ]);
  }
}
