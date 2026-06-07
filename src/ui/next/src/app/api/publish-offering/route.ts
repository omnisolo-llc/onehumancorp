import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const payload = await request.text();

    try {
        const response = await fetch(`${backendUrl}/api/v1/catalog/product`, {
            method: 'POST',
            headers: {
                'Content-Type': request.headers.get('content-type') || 'application/json',
                'Authorization': request.headers.get('authorization') || '',
                'Cookie': request.headers.get('cookie') || '',
                'X-Tenant-ID': request.headers.get('x-tenant-id') || 'default'
            },
            body: payload
        });

        const body = await response.text();
        return new NextResponse(body, {
            status: response.status,
            headers: {
                'Content-Type': response.headers.get('content-type') || 'application/json'
            }
        });
    } catch (error) {
        console.error('Publish offering backend unavailable:', error);
        return NextResponse.json({
            error: 'BACKEND_UNAVAILABLE',
            message: 'Publish offering requires the catalog backend.'
        }, { status: 503 });
    }
}
