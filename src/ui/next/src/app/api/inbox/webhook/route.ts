import { NextResponse } from 'next/server';

export async function POST(req: Request) {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const tenantId = req.headers.get('x-tenant-id') || 'default';
    const userId = req.headers.get('x-user-id') || 'default';
    const authHeader = req.headers.get('authorization');
    const headers: Record<string, string> = {
        'Content-Type': 'application/json',
        'x-tenant-id': tenantId,
        'x-user-id': userId,
    };
    if (authHeader) {
        headers.authorization = authHeader;
    }

    try {
        const body = await req.json();
        const res = await fetch(`${backendUrl}/api/v1/ai/draft-reply`, {
            method: 'POST',
            headers,
            body: JSON.stringify(body),
        });

        if (res.ok) {
            return NextResponse.json(await res.json());
        }

        return NextResponse.json({ error: 'Failed to draft inbox reply' }, { status: res.status });
    } catch {
        return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
    }
}
