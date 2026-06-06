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
    if (process.env.NODE_ENV !== "test") console.error(`Failed to fetch article ${params.articleId} from backend:`, e);
    return NextResponse.json({ error: "Internal server error" }, { status: 500 });
  }
}
