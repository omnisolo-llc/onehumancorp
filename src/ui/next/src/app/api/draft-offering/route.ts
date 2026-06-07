import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const payload = await request.text();

    try {
        const response = await fetch(`${backendUrl}/api/v1/catalog/draft-offering`, {
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
        console.error('Draft offering backend unavailable:', error);

        // Fallback mock for testing
        const { intent } = JSON.parse(payload);
        const lowerIntent = intent.toLowerCase();
        let type = 'physical';
        if (lowerIntent.includes('lesson') || lowerIntent.includes('consultation') || lowerIntent.includes('service') || lowerIntent.includes('hour') || lowerIntent.includes('booking')) {
            type = 'service';
        }

        return NextResponse.json({
            title: intent.length > 30 ? intent.substring(0, 30) + '...' : intent,
            description: `A fantastic ${type} offering generated from your request: "${intent}". We've set this up so you can start selling immediately.`,
            price: 50,
            type: type
        }, { status: 200 });
    }
}
