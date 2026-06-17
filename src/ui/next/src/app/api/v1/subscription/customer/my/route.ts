import { NextResponse } from 'next/server';

export async function GET(request: Request) {
    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    try {
        const response = await fetch(`${backendUrl}/api/subscriptions/customer/my`, {
            method: 'GET',
            headers: {
                'Authorization': request.headers.get('authorization') || '',
                'Cookie': request.headers.get('cookie') || ''
            }
        });

        const body = await response.text();
        return new NextResponse(body, {
            status: response.status,
            headers: {
                'Content-Type': response.headers.get('content-type') || 'application/json'
            }
        });
    } catch (error) {
        return NextResponse.json({ error: 'BACKEND_UNAVAILABLE' }, { status: 503 });
    }
}
