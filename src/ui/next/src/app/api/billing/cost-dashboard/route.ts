import { NextResponse } from 'next/server';

export async function GET() {
  const now = new Date();
  const startOfMonth = new Date(now.getFullYear(), now.getMonth(), 1);
  const endOfMonth = new Date(now.getFullYear(), now.getMonth() + 1, 0);

  return NextResponse.json({
      total_revenue: 50000, // $500.00
      total_costs: 2000,    // $20.00
      llm_cost: 1500,       // $15.00
      storage_cost: 0,
      payment_fees: 500,    // $5.00
      period_start: startOfMonth.toLocaleDateString('en-CA'),
      period_end: endOfMonth.toLocaleDateString('en-CA'),
  });
}
