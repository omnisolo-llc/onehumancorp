import { NextResponse } from 'next/server';

export async function GET(req: Request) {
    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    const tenantId = req.headers.get('x-tenant-id') || 'default';
    const userId = req.headers.get('x-user-id') || 'default';
    const authHeader = req.headers.get('authorization');
    const headers: Record<string, string> = {
        'x-tenant-id': tenantId,
        'x-user-id': userId,
    };
    if (authHeader) {
        headers.authorization = authHeader;
    }

    try {
        const plansRes = await fetch(`${backendUrl}/api/subscriptions/plans`, { headers });
        const subscribersRes = await fetch(`${backendUrl}/api/subscriptions/subscribers`, { headers });
        const batchesRes = await fetch(`${backendUrl}/api/subscriptions/fulfillment-batches`, { headers });

        if (plansRes.ok && subscribersRes.ok && batchesRes.ok) {
            const plans = await plansRes.json();
            const subscribers = await subscribersRes.json();
            const batches = await batchesRes.json();
            return NextResponse.json({ plans, subscribers, batches });
        }

        return NextResponse.json({ error: 'Failed to fetch subscriptions' }, { status: res.status });
    } catch {
        return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
    }
}
