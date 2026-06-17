import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  const fallbackTooltips = {
    "changelog-nav-tooltip": "See what's new in the latest updates.",
    "api-docs-tooltip": "Direct API access is only for custom integrations.",
    "inbox-activity-tooltip": "Keep track of recent customer messages. Reply or assign them to an AI agent.",
    "recent-orders-tooltip": "View the latest orders placed by your customers.",
    "total-sales-tooltip": "Total revenue generated from all your orders.",
    "kairos-nav-link-tooltip": "Click here to see what your AI helpers are working on and how they plan.",
    "dashboard-tooltip": "View your daily sales and overall business health.",
    "inventory-tooltip": "Manage your inventory, prices, and stock levels.",
    "orders-tooltip": "See what customers bought and track order fulfillment.",
    "appshell-help-btn-tooltip": "Need help? Click here to access our Help Center and tutorials.",
    "ask-ai-tooltip": "Open AI Help Chat to get answers instantly. It can guide you to the right article.",
    "pricing-tier-tooltip": "Select the plan that best fits your business needs.",
    "bio-input-tooltip": "Describe what you sell, your target audience, and the vibe of your brand.",
    "generate-btn-tooltip": "Our AI agents will analyze your description and build a ready-to-launch store for you.",
    "change-vibe-tooltip": "Change the theme and colors of your website.",
    "remove-branding-tooltip": "Upgrade to Premium to remove OHC branding.",
    "launch-btn-tooltip": "Launch your storefront immediately to a live URL.",
    "settings-verify-tooltip": "Verify your number to receive critical notifications.",
    "settings-otp-tooltip": "Click to confirm the code sent to your phone.",
    "settings-delivery-tooltip": "Turn this on to offer local delivery to your customers.",
    "morning-briefing": "Your AI Decision Assistant's daily summary.",
    "checkout-cancel-tooltip": "Go back to the previous screen without subscribing.",
    "department-card-tooltip": "Click to view and manage pending approvals for this department.",
    "my-plan-tooltip": "View and manage your subscription plan and usage."
  };

  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${backendUrl}/api/tooltips`).catch(() => null);

    if (res && res.ok) {
      const data = await res.json();
      if (data && Object.keys(data).length > 0) {
          return NextResponse.json(data);
      }
    }

    return NextResponse.json(fallbackTooltips, { status: 200 });
  } catch (e) {
    if (process.env.NODE_ENV !== "test") console.error("Failed to fetch tooltips from backend:", e);
    return NextResponse.json(fallbackTooltips, { status: 200 });
  }
}
