import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  // Try to forward the request to the rust backend.
  // We need the token to authorize
  const authHeader = request.headers.get('authorization');

  try {
    const res = await fetch(`${process.env.BACKEND_URL || 'http://localhost:8080'}/api/billing/cost-dashboard`, {
      headers: {
        'Authorization': authHeader || '',
      }
    });

    if (res.ok) {
        const data = await res.json();
        return NextResponse.json(data);
    }

    // fallback
    const now = new Date();
    const startOfMonth = new Date(now.getFullYear(), now.getMonth(), 1);
    const endOfMonth = new Date(now.getFullYear(), now.getMonth() + 1, 0);

    return NextResponse.json({
        total_revenue: 0,
        total_costs: 0,
        llm_cost: 0,
        storage_cost: 0,
        payment_fees: 0,
        period_start: startOfMonth.toLocaleDateString('en-CA'),
        period_end: endOfMonth.toLocaleDateString('en-CA'),
    });
  } catch(e) {
    console.error("Backend fetch error", e);
    const now = new Date();
    const startOfMonth = new Date(now.getFullYear(), now.getMonth(), 1);
    const endOfMonth = new Date(now.getFullYear(), now.getMonth() + 1, 0);

    return NextResponse.json({
        total_revenue: 0,
        total_costs: 0,
        llm_cost: 0,
        storage_cost: 0,
        payment_fees: 0,
        period_start: startOfMonth.toLocaleDateString('en-CA'),
        period_end: endOfMonth.toLocaleDateString('en-CA'),
    });
  }
}
