import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    try {
        const backendUrl = process.env.OHC_BACKEND_URL || 'http://127.0.0.1:18789';
        const body = await request.json();
        const action = body.action; // 'click' or 'conversion'
        const referrerId = body.referrer_id; // For simplicity in MVP, assuming this maps closely or we proxy it

        console.log(`[Referral Tracking] Action: ${action}, Referrer: ${referrerId}, Offer: ${body.offer}`);

        const headers = new Headers({
            'Content-Type': 'application/json',
        });
        const authHeader = request.headers.get('authorization');
        if (authHeader) headers.set('authorization', authHeader);
        const cookie = request.headers.get('cookie');
        if (cookie) headers.set('cookie', cookie);

        let endpoint = '';
        if (action === 'click') {
            endpoint = `${backendUrl}/api/v1/growth/referrals/click`;
        } else if (action === 'conversion') {
             endpoint = `${backendUrl}/api/v1/growth/referrals/convert`;
        }

        if (endpoint) {
            // Send to rust backend. Note: rust backend expects an ID format {"id": "..."} for GrowthIdRequest.
            // We pass referrer_id as the ID. In a real system we'd map code to id, but let's assume referrer_id = id or code for MVP flow.
            const backendRes = await fetch(endpoint, {
                method: 'POST',
                headers,
                body: JSON.stringify({ id: referrerId })
            });

            if (!backendRes.ok) {
                console.warn(`Backend tracking returned ${backendRes.status} for action ${action}`);
            }
        }

        return NextResponse.json({ success: true, tracked: true });
    } catch (e) {
        console.error('Tracking error', e);
        return NextResponse.json({ error: 'Internal error' }, { status: 500 });
    }
}
