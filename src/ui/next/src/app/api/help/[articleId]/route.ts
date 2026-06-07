import { NextResponse } from 'next/server';

export async function GET(req: Request, { params }: { params: { articleId: string } }) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${backendUrl}/api/help/${params.articleId}`);

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    } else if (res.status === 404) {
      return NextResponse.json({ error: "Article not found" }, { status: 404 });
    }

    return NextResponse.json({ error: "Backend error" }, { status: res.status });
  } catch (e) {
    if (process.env.NODE_ENV !== "test") console.error("Failed to fetch help article from backend:", e);
    return NextResponse.json({ error: "Failed to fetch from backend" }, { status: 500 });
  }
}
