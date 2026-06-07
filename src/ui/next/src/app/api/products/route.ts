import { NextResponse } from 'next/server';

export const dynamic = 'force-dynamic';

export async function GET(request: Request) {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';

    try {
        const response = await fetch(`${backendUrl}/api/v1/catalog/products`, {
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
        console.error('Products fetch backend unavailable:', error);
        return NextResponse.json({
            error: 'BACKEND_UNAVAILABLE',
            message: 'Products fetch requires the catalog backend.'
        }, { status: 503 });
    }
}
