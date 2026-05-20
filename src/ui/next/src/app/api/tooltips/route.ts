import { NextResponse } from 'next/server';

export async function GET() {
  const tooltips = {
    "bio-input-tooltip": "Describe what you sell, your target audience, and the vibe of your brand.",
    "generate-btn-tooltip": "Our AI agents will analyze your description and build a ready-to-launch store for you.",
    "launch-btn-tooltip": "Launch your storefront immediately to a live URL.",
    "dashboard-sales-tooltip": "Your total revenue for the current day.",
    "dashboard-customers-tooltip": "The number of customers currently engaging with your store.",
    "dashboard-orders-tooltip": "Orders that have been placed but not yet fulfilled.",
    "dashboard-swarm-tooltip": "Shows if your AI team is currently online and ready to help.",
    "dashboard-activity-tooltip": "A live view of what your AI agents are currently working on."
  };

  return NextResponse.json(tooltips);
}
