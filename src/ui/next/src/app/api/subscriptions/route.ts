import { NextResponse } from 'next/server';

export async function GET(req: Request) {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
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
        const res = await fetch(`${backendUrl}/api/subscriptions`, {
            method: 'GET',
            headers,
        });

        if (res.ok) {
            return NextResponse.json(await res.json());
        }

        return NextResponse.json({ error: 'Failed to fetch subscriptions' }, { status: res.status });
    } catch {
        return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
    }
}
