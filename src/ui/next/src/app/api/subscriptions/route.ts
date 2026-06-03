import { NextResponse } from 'next/server';

export async function GET(request: Request) {
    // Return mock data for subscriptions
    return NextResponse.json({
        plans: [
            { id: 'plan_1', name: 'Vegan Cake', price_cents: 5000, frequency: 'monthly', cutoff_day: 5 },
            { id: 'plan_2', name: 'Monthly Coffee Bean', price_cents: 1999, frequency: 'monthly', cutoff_day: 5 }
        ],
        subscribers: [
            { id: 'sub_1', customer_id: 'cust_1', status: 'ACTIVE' },
            { id: 'sub_2', customer_id: 'cust_2', status: 'ACTIVE' }
        ],
        batches: [
            { id: 'batch_1', fulfillment_date: '2024-06-05', subscriber_count: 2, status: 'PENDING' }
        ]
    });
}
