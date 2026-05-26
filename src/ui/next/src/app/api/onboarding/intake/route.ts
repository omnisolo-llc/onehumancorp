import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  // Mock implementation for verification if backend is not running
  if (process.env.NODE_ENV === 'development' || !process.env.BACKEND_URL) {
    return NextResponse.json({
        business_type: "Retail",
        business_name: "Maya's Bakery",
        categories: ["Cakes", "Pastries"],
        initial_products: [
            { name: "Custom Wedding Cake", price: "150.00" }
        ]
    });
  }

  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  try {
    const body = await request.json();
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
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
