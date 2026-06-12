import { NextResponse, NextRequest } from 'next/server';
import { fallbackArticles } from '../fallback';

export async function GET(
  request: NextRequest,
  { params }: { params: Promise<{ articleId: string }> }
) {
  const articleId = (await params).articleId;
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${backendUrl}/api/help/${articleId}`).catch(() => null);

    if (res && res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    // Fallback logic
    const article = fallbackArticles.find(a => a.id === articleId);
    if (article) {
       return NextResponse.json(article, { status: 200 });
    }

    if (res && res.status === 404) {
       return NextResponse.json({ error: "Article not found" }, { status: 404 });
    }

    return NextResponse.json({ error: "Article not found" }, { status: 404 });
  } catch (e) {
    if (process.env.NODE_ENV !== "test") console.error("Failed to fetch article from backend:", e);
    const article = fallbackArticles.find(a => a.id === articleId);
    if (article) {
       return NextResponse.json(article, { status: 200 });
    }
    return NextResponse.json({ error: "Internal server error" }, { status: 500 });
  }
}
