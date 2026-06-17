import { NextResponse } from 'next/server';

export async function GET(request: Request, { params }: { params: { id: string } }) {
    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    const { id } = params;

    try {
        const response = await fetch(`${backendUrl}/api/v1/catalog/product/${id}`, {
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

export async function PUT(request: Request, { params }: { params: { id: string } }) {
    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    const { id } = params;
    const payload = await request.text();

    try {
        const response = await fetch(`${backendUrl}/api/v1/catalog/product/${id}`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': request.headers.get('authorization') || '',
                'Cookie': request.headers.get('cookie') || ''
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
        return NextResponse.json({
            error: 'BACKEND_UNAVAILABLE',
            message: 'Product update requires the catalog backend.'
        }, { status: 503 });
    }
}
