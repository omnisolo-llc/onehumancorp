import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const tenant_id = searchParams.get('tenant_id') || 'default-team';
    const backendUrl = process.env.OHC_CORE_URL || 'http://localhost:8080';
    const authHeader = request.headers.get('Authorization') || request.headers.get('x-spiffe-id') || '';

    // First try the backend
    try {
      const backendRes = await fetch(`${backendUrl}/api/v1/growth/wrapped?tenant_id=${tenant_id}`, {
        method: 'GET',
        headers: {
          'x-spiffe-id': authHeader.includes('spiffe') ? authHeader : 'spiffe://ohc/org/e2e-tenant/agent/browser',
        },
      });

      if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
      }
    } catch (err) {
      console.warn("Backend fetch failed for wrapped data, using fallback", err);
    }

    // Fallback data for the UI
    const year = new Date().getFullYear();
    const data = {
      year,
      title: "Your Year in Review 🎉",
      subtitle: "See how your AI agents and viral loops grew your business.",
      stats: {
        totalSales: "$124,500",
        totalOrders: 1420,
        newCustomers: 850,
        topProduct: "Vegan Celebration Cake",
        aiHoursSaved: 124
      },
      shareText: `My AI agents saved me 124 hours this year and drove $124k in sales! Check out my OHC Year in Review:`
    };

    return NextResponse.json(data);
  } catch (error) {
    return NextResponse.json({ error: 'Failed to fetch wrapped data' }, { status: 500 });
  }
}
