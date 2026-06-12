import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const tenantId = searchParams.get('tenant_id') || 'default';

  try {
    const backendUrl = process.env.NEXT_PUBLIC_API_URL || 'http://127.0.0.1:18789';
    const headers = new Headers({ 'Content-Type': 'application/json' });

    // Ensure we do NOT leak auth headers for a public API route.
    // If the backend requires auth for `/api/pos/orders` in some deployments, we provide
    // a service token if configured, otherwise we fetch unauthenticated (which works locally but may fail in prod).
    // The key here is not proxying the user's cookies/auth for a public widget!
    if (process.env.INTERNAL_SERVICE_TOKEN) {
        headers.set('authorization', `Bearer ${process.env.INTERNAL_SERVICE_TOKEN}`);
    }

    const backendRes = await fetch(`${backendUrl}/api/pos/orders?tenant_id=${tenantId}`, {
      method: 'GET',
      headers,
    });

    if (backendRes.ok) {
        const data = await backendRes.json();

        // Ensure absolutely no PII is leaked to the public unauthenticated widget
        const sanitizedOrders = (data.orders || []).map((o: any) => ({
            id: o.id,
            total_amount: o.total_amount,
            status: "recent_purchase", // mask internal status
            created_at: o.created_at
        }));

        return NextResponse.json({ orders: sanitizedOrders });
    } else {
        // Fallback to recent generic metrics or empty array if unauthorized
        return NextResponse.json({ orders: [] }, { status: backendRes.status });
    }
  } catch (error) {
    if (process.env.NODE_ENV !== "test") console.error("Error fetching social proof orders:", error);
    return NextResponse.json({ orders: [] }, { status: 500 });
  }
}
