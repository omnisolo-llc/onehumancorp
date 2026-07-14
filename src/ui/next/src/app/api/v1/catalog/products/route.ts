import { NextResponse } from 'next/server';

export async function GET(request: Request) {
    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    try {
        const response = await fetch(`${backendUrl}/api/v1/catalog/products`, {
            method: 'GET',
            headers: {
                'Content-Type': request.headers.get('content-type') || 'application/json',
                'Authorization': request.headers.get('authorization') || '',
                'Cookie': request.headers.get('cookie') || ''
            },
        });
        const body = await response.text();
        return new NextResponse(body, {
            status: response.status,
            headers: {
                'Content-Type': response.headers.get('content-type') || 'application/json'
            }
        });
    } catch (e) {
        return NextResponse.json({ error: "Backend unavailable" }, { status: 503 });
    }
}

export async function POST(request: Request) {
    // Actually hit the real backend if we need to update/create
    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    const payload = await request.text();
    try {
        // Just send it to the product creation endpoint as a mock "update" for now or handle appropriately
        const response = await fetch(`${backendUrl}/api/v1/catalog/product`, {
            method: 'POST',
            headers: {
                'Content-Type': request.headers.get('content-type') || 'application/json',
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
    } catch (e) {
        return NextResponse.json({ error: "Backend unavailable" }, { status: 503 });
    }
}
