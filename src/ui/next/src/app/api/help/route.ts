import { NextResponse, NextRequest } from 'next/server';

export const fallbackArticles = [
  { category: "Getting Started", id: "getting-started-1", title: "Getting Started with Your Store", desc: "Welcome to OneHumanCorp! Let's get your business online in under 10 minutes.", link: "/help/getting-started-1" },
  { category: "My Store", id: "add-products", title: "Adding Products", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/add-products" },
  { category: "Payments", id: "accept-payments", title: "Accepting Payments", desc: "Learn how to accept credit cards and manage your payouts.", link: "/help/accept-payments" },
  { category: "AI Agents", id: "ai-support", title: "Activate AI Support", desc: "Let our AI handle customer inquiries and triage your inbox.", link: "/help/ai-support" },
  { category: "Marketing", id: "marketing-tools", title: "Grow Your Audience", desc: "Use our built-in tools to run promotions and track performance.", link: "/help/marketing-tools" },
  { category: "Account & Billing", id: "billing-settings", title: "Manage Billing", desc: "Update your subscription and payment methods.", link: "/help/billing-settings" },
];

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${backendUrl}/api/help`).catch(() => null);

    if (res && res.ok) {
      const data = await res.json();
      if (data && data.length > 0) {
        return NextResponse.json(data);
      }
    }

    // Always fallback if empty or failed
    return NextResponse.json(fallbackArticles, { status: 200 });
  } catch (e) {
    if (process.env.NODE_ENV !== "test" && process.env.CI !== "1") {
      console.error("Failed to fetch help from backend:", e);
    }
    return NextResponse.json(fallbackArticles, { status: 200 });
  }
}
