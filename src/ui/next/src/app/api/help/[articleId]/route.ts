import { NextResponse, NextRequest } from 'next/server';

export async function GET(req: NextRequest, { params }: { params: { articleId: string } }) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';

  try {
    const res = await fetch(`${backendUrl}/api/help/${params.articleId}`);

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({ error: "Article not found" }, { status: res.status });
  } catch (e) {
    console.error("Failed to fetch help article from backend:", e);
    return NextResponse.json({ error: "Article not found" }, { status: 500 });
  }
}
