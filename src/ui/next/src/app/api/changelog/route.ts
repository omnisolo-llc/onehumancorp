import { NextResponse, NextRequest } from 'next/server';

export const dynamic = 'force-dynamic';

const fallbackChangelog = [
  {
    version: "v1.0.0",
    contentLines: [
      "### Initial Release",
      "- Welcome to OneHumanCorp!",
      "- Added AI Support Agent to help manage your business.",
      "- Included Storefront Builder for quick setup.",
    ]
  }
];

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${backendUrl}/api/changelog`);

    if (res.ok) {
      const data = await res.json();
      if (data && data.length > 0) {
        return NextResponse.json(data);
      }
    }

    return NextResponse.json(fallbackChangelog, { status: 200 });
  } catch (e) {
    if (process.env.NODE_ENV !== "test" && process.env.CI !== "1") {
      console.error("Failed to fetch changelog from backend:", e);
    }
    return NextResponse.json(fallbackChangelog, { status: 200 });
  }
}
