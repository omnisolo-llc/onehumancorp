import { NextResponse, NextRequest } from 'next/server';

const fallbackVideos = [
  { id: 1, title: "How to set up your first store easily", duration: "1:15", video_url: "https://www.w3schools.com/html/mov_bbb.mp4" },
  { id: 2, title: "Connecting a bank account to accept payments", duration: "0:45", video_url: "https://www.w3schools.com/html/mov_bbb.mp4" },
  { id: 3, title: "Activating your AI Support Agent", duration: "1:25", video_url: "https://www.w3schools.com/html/mov_bbb.mp4" },
  { id: 4, title: "Adding a new product to your inventory", duration: "0:50", video_url: "https://www.w3schools.com/html/mov_bbb.mp4" },
  { id: 5, title: "Managing staff and user permissions", duration: "1:10", video_url: "https://www.w3schools.com/html/mov_bbb.mp4" },
  { id: 6, title: "Creating a marketing campaign", duration: "1:30", video_url: "https://www.w3schools.com/html/mov_bbb.mp4" },
  { id: 7, title: "Using the Analytics Dashboard", duration: "1:20", video_url: "https://www.w3schools.com/html/mov_bbb.mp4" },
  { id: 8, title: "How to handle refunds and returns", duration: "1:05", video_url: "https://www.w3schools.com/html/mov_bbb.mp4" },
  { id: 9, title: "Customizing your storefront design", duration: "1:15", video_url: "https://www.w3schools.com/html/mov_bbb.mp4" },
  { id: 10, title: "Setting up automated email receipts", duration: "0:55", video_url: "https://www.w3schools.com/html/mov_bbb.mp4" }
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
