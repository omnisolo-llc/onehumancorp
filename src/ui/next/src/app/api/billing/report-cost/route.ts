import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    const authHeader = request.headers.get('Authorization');
    const tenantId = request.headers.get('x-tenant-id');

    const backendUrl = process.env.NEXT_PUBLIC_API_URL || process.env.BACKEND_URL || 'http://127.0.0.1:18789';

    try {
        const body = await request.json();

        const headers: Record<string, string> = {
            'Content-Type': 'application/json',
        };
        if (authHeader) headers['Authorization'] = authHeader;
        if (tenantId) headers['x-tenant-id'] = tenantId;

        const response = await fetch(`${backendUrl}/api/billing/report-cost`, {
            method: 'POST',
            headers,
            body: JSON.stringify(body),
        });

        if (!response.ok) {
            console.error('Failed to fetch from backend', response.status);
            return NextResponse.json({ error: 'Failed to report cost to backend' }, { status: response.status === 404 ? 404 : 502 });
        }

        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            const data = await response.json();
            return NextResponse.json(data);
        } else {
            const text = await response.text();
            return new NextResponse(text, { status: 200, headers: { 'Content-Type': contentType || 'text/plain' } });
        }
    } catch (error) {
        console.error('Error proxying report cost to backend', error);
        return NextResponse.json({ error: 'Error proxying to backend' }, { status: 502 });
    }
}
