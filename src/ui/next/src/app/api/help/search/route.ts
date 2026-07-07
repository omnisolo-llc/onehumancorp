import { NextResponse, NextRequest } from "next/server";

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";
  const { searchParams } = new URL(request.url);
  const q = searchParams.get('q') || '';

  try {
    const res = await fetch(`${backendUrl}/api/help/search?q=${encodeURIComponent(q)}`).catch(() => null);

    if (res && res.ok) {
      const data = await res.json();
      if (data && data.length > 0) {
        return NextResponse.json(data);
      }
    }

    // Fallback static data
    const all = [
      { category: "Getting Started", title: "Getting Started with Your Store", desc: "Welcome to OneHumanCorp! Let's get your business online in under 10 minutes.", link: "/help/getting-started-1" },
      { category: "My Store", title: "Adding Products", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/add-products" },
      { category: "Payments", title: "Accepting Payments", desc: "Learn how to accept credit cards and manage your payouts.", link: "/help/accept-payments" },
    ];

    return NextResponse.json(all.filter(a => a.title.toLowerCase().includes(q.toLowerCase()) || a.desc.toLowerCase().includes(q.toLowerCase())));
  } catch (e) {
    return NextResponse.json([]);
  }
}
