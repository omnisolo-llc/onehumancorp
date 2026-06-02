import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  try {
    const body = await request.json();

    if (process.env.NODE_ENV === 'development' || process.env.NEXT_PUBLIC_MOCK === 'true') {
        const description = body.description || "";
        const nameMatch = description.match(/Business Name: (.*)\n/);
        const name = nameMatch ? nameMatch[1] : "Elena's Ethos";

        return NextResponse.json({
            business_name: name,
            business_type: "Artisanal Candle Shop",
            categories: ["physical", "handmade"],
            initial_products: [
                { name: "Midnight Jasmine Candle", price: "24.99" },
                { name: "Sedona Sunset Candle", price: "24.99" }
            ]
        });
    }

    const res = await fetch(`${backendUrl}/api/onboarding/intake`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-tenant-id': tenantId,
        'x-user-id': userId
      },
      body: JSON.stringify(body)
    });

    if (res.ok) {
        const data = await res.json();
        return NextResponse.json(data);
    }

    return NextResponse.json({ error: 'Failed to process intake' }, { status: res.status });
  } catch (e) {
    // Fallback for demo/dev if backend is down
    if (process.env.NODE_ENV === 'development') {
       return NextResponse.json({
            business_name: "Elena's Ethos",
            business_type: "Artisanal Candle Shop",
            categories: ["physical", "handmade"],
            initial_products: [
                { name: "Midnight Jasmine Candle", price: "24.99" }
            ]
        });
    }
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
