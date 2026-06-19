import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    try {
        const backendUrl = process.env.OHC_API_URL || 'http://localhost:8080';

        const authHeader = request.headers.get('authorization');
        const headers: Record<string, string> = {
            'Content-Type': 'application/json',
        };
        if (authHeader) {
            headers['Authorization'] = authHeader;
        }

        const res = await fetch(`${backendUrl}/api/billing/create-billing-portal-session`, {
            method: 'POST',
            headers,
        });

        if (!res.ok) {
            console.error('Failed to create billing portal session', res.status);
            return NextResponse.json({ error: 'Failed to create billing portal session' }, { status: res.status });
        }

        const data = await res.json();
        return NextResponse.json(data);
    } catch (error) {
        console.error('Error creating billing portal session:', error);
        return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
    }
}
