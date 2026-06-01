import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  let body: any = {};
  try {
    const text = await request.text();
    if (text) {
      body = JSON.parse(text);
    }
  } catch (e) {
    console.error("Failed to parse body:", e);
  }

  try {
    // In production, BACKEND_URL would be defined. For local dev we use the default 8080.
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/promotions/generate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        // Fallback if backend is not available
        const { tenant } = body;
        const tenantStr = tenant || 'our store';
        const message = `🎉 Exciting news from ${tenantStr}!\n\nAs a special thank you to our amazing community, we are running a limited-time promotion.\n\nUse code **SPECIAL15** at checkout to get 15% off your next order.\n\nHurry, this offer won't last long!\n\nShop now: https://ohc.store/${tenantStr}\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`;
        return NextResponse.json({ message });
    }
  } catch (error) {
    console.error("Error generating promotion:", error);
    // Fallback if fetch fails completely
    const { tenant } = body;
    const tenantStr = tenant || 'our store';
    const message = `🎉 Exciting news from ${tenantStr}!\n\nAs a special thank you to our amazing community, we are running a limited-time promotion.\n\nUse code **SPECIAL15** at checkout to get 15% off your next order.\n\nHurry, this offer won't last long!\n\nShop now: https://ohc.store/${tenantStr}\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`;
    return NextResponse.json({ message });
  }
}
