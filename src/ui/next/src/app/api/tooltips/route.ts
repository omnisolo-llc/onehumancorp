import { NextResponse } from 'next/server';

export async function GET() {
  const tooltips = {
    "bio-input-tooltip": "Describe what you sell, your target audience, and the vibe of your brand.",
    "generate-btn-tooltip": "Our AI agents will analyze your description and build a ready-to-launch store for you.",
    "launch-btn-tooltip": "Launch your storefront immediately to a live URL."
  };

  return NextResponse.json(tooltips);
}
