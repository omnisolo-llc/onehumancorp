import { NextResponse, NextRequest } from "next/server";

export async function GET(request: NextRequest, { params }: { params: { articleId: string } }) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";

  try {
    const res = await fetch(`${backendUrl}/api/help/${params.articleId}`).catch(() => null);

    if (res && res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    if (params.articleId === "getting-started-1") {
        return NextResponse.json({
            title: "Getting Started with Your Store",
            contentHtml: "<p>Welcome to OneHumanCorp! Setting up your store is quick and easy. Our app helps you get everything ready to sell online.</p><h2>Step 1: Tell us about your business</h2>"
        });
    }

    return NextResponse.json({ error: "Article not found" }, { status: 404 });
  } catch (e) {
    return NextResponse.json({ error: "Article not found" }, { status: 404 });
  }
}
