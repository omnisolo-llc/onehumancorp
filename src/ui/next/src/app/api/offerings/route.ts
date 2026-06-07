import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';

    try {
        const body = await request.json();
        const intent = body.intent || '';

        // Temporary AI mock until we route this properly to the Go backend via MCP/API
        const generatedData = {
          title: intent.trim().split(' ').map((w: string) => w.charAt(0).toUpperCase() + w.slice(1)).join(' '),
          description: `Experience the best ${intent.toLowerCase()}. Tailored to your needs and designed to deliver an exceptional outcome.`,
          type: intent.toLowerCase().includes('lesson') || intent.toLowerCase().includes('service') ? 'Service' : 'Product',
          price: '50.00',
          image: 'placeholder'
        };

        // Here we *would* do the actual product save if not mocked.
        // await fetch(`${backendUrl}/api/v1/catalog/product`, ...)

        return NextResponse.json(generatedData, { status: 200 });
    } catch (error) {
        console.error('Offering create backend unavailable:', error);
        return NextResponse.json({
            error: 'BACKEND_UNAVAILABLE',
            message: 'Offering creation failed.'
        }, { status: 503 });
    }
}
