import { NextResponse } from 'next/server';

export async function POST(req: Request, { params }: { params: Promise<{id: string}> }) {
    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    const tenantId = req.headers.get('x-tenant-id') || 'default';
    const authHeader = req.headers.get('authorization');
    const headers: Record<string, string> = {
        'x-tenant-id': tenantId,
        'Content-Type': 'application/json',
    };
    if (authHeader) {
        headers.authorization = authHeader;
    }

    try {
        const body = await req.json();
        const res = await fetch(`${backendUrl}/api/subscriptions/${(await params).id}/action`, {
            method: 'POST',
            headers,
            body: JSON.stringify(body),
        });

        if (res.ok) {
            return NextResponse.json(await res.json());
        }

        return NextResponse.json({ error: 'Failed to update subscription' }, { status: res.status });
    } catch {
        return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
    }
}
