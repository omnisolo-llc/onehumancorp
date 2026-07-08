import { NextResponse } from 'next/server';

export async function GET(request: Request) {
    const authHeader = request.headers.get('Authorization');
    const tenantId = request.headers.get('x-tenant-id');

    const backendUrl = process.env.NEXT_PUBLIC_API_URL || process.env.BACKEND_URL || 'http://127.0.0.1:18789';

    try {
        const headers: Record<string, string> = {};
        if (authHeader) headers['Authorization'] = authHeader;
        if (tenantId) headers['x-tenant-id'] = tenantId;

        const response = await fetch(`${backendUrl}/api/billing/cost-dashboard`, {
            headers
        });

        if (!response.ok) {
            console.error('Failed to fetch from backend', response.status);
            return NextResponse.json({ error: 'Failed to fetch from backend' }, { status: response.status === 404 ? 404 : 502 });
        }

        const data = await response.json();
        return NextResponse.json(data);
    } catch (error) {
        console.error('Error proxying to backend', error);
        return NextResponse.json({ error: 'Error proxying to backend' }, { status: 502 });
    }
}
