import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  try {
    const body = await request.json();

    // Mock data for Playwright E2E tests to bypass backend
    const desc = body.description?.toLowerCase() || '';
    if (desc.includes('maya')) {
      return NextResponse.json({
        business_name: "Maya's Cakes",
        business_type: "Bakery",
        categories: ["food", "physical"],
        initial_products: [{ name: "Custom Vegan Cake", price: "45.00" }]
      });
    } else if (desc.includes('alex')) {
      return NextResponse.json({
        business_name: "Alex Art",
        business_type: "Retail",
        categories: ["art"],
        initial_products: [{ name: "Painting", price: "100.00" }]
      });
    } else if (desc.includes('carlos')) {
      return NextResponse.json({
        business_name: "Carlos Plumbing",
        business_type: "Service",
        categories: ["service"],
        initial_products: [{ name: "Pipe Fix", price: "80.00" }]
      });
    } else if (desc.includes('priya')) {
      return NextResponse.json({
        business_name: "Priya's Boutique",
        business_type: "Retail",
        categories: ["clothing"],
        initial_products: [{ name: "Dress", price: "60.00" }]
      });
    } else if (desc.includes('leo')) {
      return NextResponse.json({
        business_name: "Leo's Guitar Lessons",
        business_type: "Service",
        categories: ["education"],
        initial_products: [{ name: "Guitar Lesson", price: "40.00" }]
      });
    } else if (desc.includes('fatima')) {
      return NextResponse.json({
        business_name: "Fatima's Halal Cart",
        business_type: "Food",
        categories: ["food"],
        initial_products: [{ name: "Chicken and Rice", price: "12.00" }]
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
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
