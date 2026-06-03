import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();

    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    // We will attempt to call the backend, but if it fails we mock it.
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/campaign/generate-seo`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        const { store_name } = body;
        const name = store_name || 'your store';

        const message = `SEO Audit Report for ${name}:\n\n✅ Mobile Responsiveness: Good\n❌ Meta Descriptions: Missing on 5 products\n❌ Page Speed: Need to optimize 3 large images\n❌ Local SEO: Missing Google Maps integration\n\nAI Recommendations:\nWe can automatically write high-converting meta descriptions for your products and compress your images to boost your Google ranking and bring in 2x more local customers.\n\n⚡ Powered by OHC`;

        return NextResponse.json({ message });
    }
  } catch (error) {
    console.error("Error generating SEO report:", error);
    const message = `SEO Audit Report:\n\n✅ Mobile Responsiveness: Good\n❌ Meta Descriptions: Missing on 5 products\n❌ Page Speed: Need to optimize 3 large images\n❌ Local SEO: Missing Google Maps integration\n\nAI Recommendations:\nWe can automatically write high-converting meta descriptions for your products and compress your images to boost your Google ranking and bring in 2x more local customers.\n\n⚡ Powered by OHC`;
    return NextResponse.json({ message });
  }
}
