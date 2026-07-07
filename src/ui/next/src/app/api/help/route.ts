import { NextResponse, NextRequest } from "next/server";

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";

  try {
    const res = await fetch(`${backendUrl}/api/help`).catch(() => null);

    if (res && res.ok) {
      const data = await res.json();
      if (data && data.length > 0) {
        return NextResponse.json(data);
      }
    }

    // Fallback to static mock data if backend is down
    return NextResponse.json([
      { category: "Getting Started", title: "Getting Started with Your Store", desc: "Welcome to OneHumanCorp! Let's get your business online in under 10 minutes.", link: "/help/getting-started-1" },
      { category: "My Store", title: "Adding Products", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/add-products" },
      { category: "Payments", title: "Accepting Payments", desc: "Learn how to accept credit cards and manage your payouts.", link: "/help/accept-payments" },
      { category: "AI Agents", title: "Activate AI Support", desc: "Let our AI handle customer inquiries and triage your inbox.", link: "/help/ai-support" },
      { category: "Marketing", title: "Grow Your Audience", desc: "Use our built-in tools to run promotions and track performance.", link: "/help/marketing-tools" },
      { category: "Account & Billing", title: "Manage Billing", desc: "Update your subscription and payment methods.", link: "/help/billing-settings" },
      { category: "Advanced", title: "API Documentation", desc: "Interactive API reference for connecting external services to your workspace.", link: "/api-docs" },
    ]);
  } catch (e) {
    return NextResponse.json([
      { category: "Getting Started", title: "Getting Started with Your Store", desc: "Welcome to OneHumanCorp! Let's get your business online in under 10 minutes.", link: "/help/getting-started-1" },
      { category: "My Store", title: "Adding Products", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/add-products" },
      { category: "Payments", title: "Accepting Payments", desc: "Learn how to accept credit cards and manage your payouts.", link: "/help/accept-payments" }
    ]);
  }
}
