import { NextResponse } from 'next/server';

export async function GET(request: Request) {
    // In a real environment, this route could proxy to the Rust backend (e.g. `http://127.0.0.1:18789/api/subscriptions`).
    // Since this is a legacy Next.js prototype and the Rust backend manages its own routes directly, we fetch from it.

    try {
        const plansRes = await fetch('http://127.0.0.1:18789/api/subscriptions/plans', { headers: request.headers });
        const subscribersRes = await fetch('http://127.0.0.1:18789/api/subscriptions/subscribers', { headers: request.headers });
        const batchesRes = await fetch('http://127.0.0.1:18789/api/subscriptions/fulfillment-batches', { headers: request.headers });

        let plans = [];
        let subscribers = [];
        let batches = [];

        if (plansRes.ok) {
            plans = await plansRes.json();
        }
        if (subscribersRes.ok) {
            subscribers = await subscribersRes.json();
        }
        if (batchesRes.ok) {
            batches = await batchesRes.json();
        }

        // Return real data from the rust backend, mapping field names to what the frontend expects
        return NextResponse.json({
            plans: plans.map((p: any) => ({
                 id: p.id,
                 name: p.name,
                 price_cents: p.amount,
                 frequency: p.interval,
                 cutoff_day: 5
            })),
            subscribers: subscribers,
            batches: batches.map((b: any) => ({
                 id: b.id,
                 fulfillment_date: new Date(b.target_date * 1000).toISOString().split('T')[0],
                 subscriber_count: b.subscriber_count,
                 status: b.status
            }))
        });

    } catch (e) {
        console.error("Failed to fetch from Rust backend. Falling back to mock data for dev.", e);
        // Fallback for development if rust server isn't up
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
}
