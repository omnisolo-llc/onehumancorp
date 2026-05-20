import { NextResponse } from 'next/server';

export async function GET() {
  const tooltips = {
    "bio-input-tooltip": "Describe what you sell, your target audience, and the vibe of your brand.",
    "generate-btn-tooltip": "Our AI agents will analyze your description and build a ready-to-launch store for you.",
    "launch-btn-tooltip": "Launch your storefront immediately to a live URL.",
    "sales-tooltip": "Your total revenue for today. Updates in real-time as orders are paid.",
    "customers-tooltip": "People who have visited your store and interacted with your products today.",
    "orders-tooltip": "Orders that have been placed but not yet fulfilled by you."
  };

  return NextResponse.json(tooltips);
}
