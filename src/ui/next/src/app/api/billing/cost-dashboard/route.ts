import { NextResponse } from 'next/server';

export async function GET(request: Request) {
    const now = new Date();
    const startOfMonth = new Date(now.getFullYear(), now.getMonth(), 1);
    const endOfMonth = new Date(now.getFullYear(), now.getMonth() + 1, 0);

    return NextResponse.json({
        total_revenue: 12500,
        total_costs: 450,
        llm_cost: 250,
        storage_cost: 50,
        payment_fees: 150,
        period_start: startOfMonth.toLocaleDateString('en-CA'),
        period_end: endOfMonth.toLocaleDateString('en-CA'),
    });
}
