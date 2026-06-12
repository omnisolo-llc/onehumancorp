import { NextResponse, NextRequest } from 'next/server';
import { fallbackArticles } from '../fallback';

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const q = searchParams.get('q')?.toLowerCase() || '';

  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${backendUrl}/api/help/search?q=${encodeURIComponent(q)}`).catch(() => null);

    if (res && res.ok) {
      const data = await res.json();
      if (data && data.length > 0) {
        return NextResponse.json(data);
      }
    }

    // Fallback logic
    const results = fallbackArticles.filter(a =>
      a.title.toLowerCase().includes(q) ||
      a.desc.toLowerCase().includes(q) ||
      (a.category && a.category.toLowerCase().includes(q))
    );
    return NextResponse.json(results, { status: 200 });

  } catch (e) {
    if (process.env.NODE_ENV !== "test") console.error("Failed to fetch help search from backend:", e);
    const results = fallbackArticles.filter(a =>
      a.title.toLowerCase().includes(q) ||
      a.desc.toLowerCase().includes(q) ||
      (a.category && a.category.toLowerCase().includes(q))
    );
    return NextResponse.json(results, { status: 200 });
  }
}
