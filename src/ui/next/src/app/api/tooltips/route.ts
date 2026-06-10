import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  const fallbackTooltips = {
    "changelog-nav-tooltip": "See what's new in the latest updates.",
    "api-docs-tooltip": "Direct API access is only for custom integrations.",
    "inbox-activity-tooltip": "Keep track of recent customer messages.",
    "recent-orders-tooltip": "View the latest orders placed by your customers.",
    "total-sales-tooltip": "Total revenue generated from database orders.",
    "kairos-nav-link-tooltip": "Click here to see what your AI helpers are working on and how they plan.",
    "dashboard-tooltip": "View your daily sales and overall business health.",
    "inventory-tooltip": "Manage your inventory, prices, and stock levels.",
    "orders-tooltip": "See what customers bought and track order fulfillment.",
    "help-btn-tooltip": "Need help? Click here to access our Help Center and tutorials."
  };

  if (process.env.NODE_ENV === 'production' && !process.env.BACKEND_URL) {
    return NextResponse.json(fallbackTooltips);
  }
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${backendUrl}/api/tooltips`);

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json(fallbackTooltips, { status: 200 });
  } catch (e) {
    if (process.env.NODE_ENV !== "test") console.error("Failed to fetch tooltips from backend:", e);
    return NextResponse.json(fallbackTooltips, { status: 200 });
  }
}
