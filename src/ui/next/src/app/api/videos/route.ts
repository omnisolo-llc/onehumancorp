import { NextResponse, NextRequest } from 'next/server';

const fallbackVideos = [
  { id: 1, title: "How to set up your first store easily", duration: "1:20", video_url: "/videos/1.mp4" },
  { id: 2, title: "Accept your first payment", duration: "1:15", video_url: "/videos/2.mp4" },
  { id: 3, title: "Activate your AI Support Agent", duration: "0:50", video_url: "/videos/3.mp4" },
  { id: 4, title: "Adding staff to your account", duration: "1:05", video_url: "/videos/4.mp4" },
  { id: 5, title: "Review an order", duration: "1:10", video_url: "/videos/5.mp4" },
  { id: 6, title: "Send a campaign", duration: "1:25", video_url: "/videos/6.mp4" },
  { id: 7, title: "Connect Stripe", duration: "1:30", video_url: "/videos/7.mp4" },
  { id: 8, title: "Manage inventory", duration: "1:00", video_url: "/videos/8.mp4" },
  { id: 9, title: "How to use the OpenAPI spec", duration: "3:45", video_url: "/videos/9.mp4" },
  { id: 10, title: "View analytics and reports", duration: "1:20", video_url: "/videos/10.mp4" },
];

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${backendUrl}/api/videos`).catch(() => null);

    if (res && res.ok) {
      const data = await res.json();
      if (data && data.length > 0) {
        return NextResponse.json(data);
      }
    }

    return NextResponse.json(fallbackVideos, { status: 200 });
  } catch (e) {
    if (process.env.NODE_ENV !== "test" && process.env.CI !== "1") {
      console.error("Failed to fetch videos from backend:", e);
    }
    return NextResponse.json(fallbackVideos, { status: 200 });
  }
}
