import { NextResponse } from 'next/server';

export async function GET() {
  const tooltips = {
    "bio-input-tooltip": "Describe what you sell, your target audience, and the vibe of your brand.",
    "generate-btn-tooltip": "Our AI agents will analyze your description and build a ready-to-launch store for you.",
    "launch-btn-tooltip": "Launch your storefront immediately to a live URL.",
    "team-activity-tooltip": "Monitor the real-time actions and tasks being performed by your AI workforce.",
    "referral-tooltip": "Share your unique link to earn credits when friends join OHC.",
    "swarm-online-tooltip": "Your AI workforce is currently active and processing tasks in the background.",
    "department-card-tooltip": "Click to view and manage pending approvals for this department.",
  };

  return NextResponse.json(tooltips);
}
