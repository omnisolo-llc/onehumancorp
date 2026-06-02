import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json([
    {
      version: "Version 1.0 (Latest)",
      date: "2024-05-20",
      contentLines: [
        "### 🌟 New Features",
        "- **Interactive AI Store Builder:** You can now generate a complete storefront from just a short description of your business. AI will handle the layout and copy for you.",
        "- **Smart Tooltips:** We added helpful text bubbles to all major buttons to help you learn the system faster.",
        "- **Help Center Upgrade:** Find answers instantly with our new searchable Help Center.",
        "### 🛠️ Improvements",
        "- Faster loading times for product images.",
        "- Simplified checkout process for your customers."
      ],
      imageUrl: "/dashboard_with_charts.png"
    }
  ]);
}
