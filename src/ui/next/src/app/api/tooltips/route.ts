import { NextResponse } from 'next/server';

export async function GET() {
  const tooltips = {
    "bio-input-tooltip": "Describe what you sell, your target audience, and the vibe of your brand.",
    "generate-btn-tooltip": "Our AI agents will analyze your description and build a ready-to-launch store for you.",
    "launch-btn-tooltip": "Launch your storefront immediately to a live URL.",
    "nav-store": "This is where you manage what you sell. Add or edit products here.",
    "nav-agents": "These are your digital employees. They can talk to customers and do tasks for you.",
    "btn-new-product": "Click here to add something new to sell. You can add a photo and a price.",
    "setting-multitenant": "Runs your app on our fast servers. This is best for most businesses.",
    "setting-standalone": "Runs entirely on your computer. Great if you don't have internet."
  };

  return NextResponse.json(tooltips);
}
