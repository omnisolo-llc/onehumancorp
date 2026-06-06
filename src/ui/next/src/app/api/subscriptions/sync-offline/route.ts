import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    try {
        const body = await request.json();
        const res = await fetch('http://127.0.0.1:18789/api/subscriptions/sync-offline', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                ...request.headers
            },
            body: JSON.stringify(body)
        });

        if (res.ok) {
            return NextResponse.json({ success: true });
        } else {
            return NextResponse.json({ success: false }, { status: res.status });
        }
    } catch (e) {
        console.error("Failed to sync offline events to backend", e);
        return NextResponse.json({ success: false }, { status: 500 });
    }
}
