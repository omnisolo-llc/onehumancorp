import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  if (process.env.NODE_ENV === 'production' && !process.env.BACKEND_URL) {
    return NextResponse.json({
      "changelog-nav-tooltip": "See what's new in the latest OneHumanCorp updates.",
      "api-docs-tooltip": "Direct API access is only for custom integrations.",
    });
  }
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${backendUrl}/api/tooltips`);

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({}, { status: res.status });
  } catch (e) {
    if (process.env.NODE_ENV !== "test") console.error("Failed to fetch tooltips from backend:", e);
    return NextResponse.json({}, { status: 500 });
  }
}
