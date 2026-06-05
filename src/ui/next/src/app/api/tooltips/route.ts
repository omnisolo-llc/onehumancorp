import { NextResponse, NextRequest } from 'next/server';

export const dynamic = 'force-dynamic';

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';

  const fallbackTooltips = {
    "bio-input-tooltip": "Tell us what you sell and who your customers are. Keep it simple!",
    "generate-btn-tooltip": "Click here to have our AI build your ready-to-launch store.",
    "launch-btn-tooltip": "Make your store live on the internet so customers can visit.",
    "team-activity-tooltip": "See exactly what your AI helpers are doing right now.",
    "referral-tooltip": "Share this link with friends. You earn credits if they sign up!",
    "swarm-online-tooltip": "Your AI helpers are working hard on your tasks right now.",
    "department-card-tooltip": "Click here to see tasks that need your approval.",
    "nav-dashboard-tooltip": "Check your sales, recent orders, and how your store is doing.",
    "nav-agents-tooltip": "See your AI team, give them tasks, or hire new helpers.",
    "nav-setup-tooltip": "Set up your business info, logo, and how you get paid.",
    "credit-tooltip": "Get free credits for premium tools by inviting a friend.",
    "help-btn-tooltip": "Need help? Click here for guides, videos, and to ask our AI.",
    "changelog-nav-tooltip": "See the latest updates and new features we just added.",
    "stripe-setup-tooltip": "Connect your bank account securely with Stripe to start getting paid.",
    "checkout-pay-now-tooltip": "Click here to securely finish your purchase and process your payment.",
    "checkout-tap-to-pay-tooltip": "Tap your card or phone on the reader to pay in person.",
    "checkout-cancel-tooltip": "Go back to the previous screen without buying anything.",
    "kairos-nav-link-tooltip": "Click here to see what your AI helpers are working on and how they plan.",
    "total-sales-tooltip": "Total revenue generated from your sales today.",
    "visitors-tooltip": "Number of unique visitors who viewed your store today.",
    "agents-tab-tooltip": "Hire and manage your AI assistants here.",
    "walkthrough-btn-tooltip": "Start an interactive guide to learn how to use OHC."
  };

  try {
    const res = await fetch(`${backendUrl}/api/tooltips`);
    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }
    // Fallback if backend is missing/down
    return NextResponse.json(fallbackTooltips);
  } catch (e) {
    console.error("Failed to fetch tooltips from backend:", e);
    return NextResponse.json(fallbackTooltips);
  }
}

export async function POST(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';

  try {
    const body = await request.json();
    const res = await fetch(`${backendUrl}/api/tooltips`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body)
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({ success: false }, { status: res.status });
  } catch (e) {
    console.error("Failed to post tooltips to backend:", e);
    return NextResponse.json({ success: false }, { status: 500 });
  }
}
