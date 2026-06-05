import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    const catalogUrl = process.env.OHC_AUTO_CATALOG_URL;

    if (!catalogUrl) {
        return NextResponse.json({
            error: 'AUTO_CATALOG_UNAVAILABLE',
            message: 'Auto-catalog requires a configured catalog extraction service.'
        }, { status: 501 });
    }

    try {
        const response = await fetch(catalogUrl, {
            method: 'POST',
            headers: {
                'Content-Type': request.headers.get('content-type') || 'application/json',
                'Authorization': request.headers.get('authorization') || ''
            },
            body: await request.arrayBuffer()
        });

        const body = await response.text();
        return new NextResponse(body, {
            status: response.status,
            headers: {
                'Content-Type': response.headers.get('content-type') || 'application/json'
            }
        });
    } catch (error) {
        console.error('Auto-catalog service unavailable:', error);
        return NextResponse.json({
            error: 'AUTO_CATALOG_UNAVAILABLE',
            message: 'Auto-catalog requires a reachable catalog extraction service.'
        }, { status: 503 });
    }
}
