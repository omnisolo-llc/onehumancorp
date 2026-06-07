import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { intent } = await request.json();

    const backendUrl = process.env.OHC_MAIN_SERVER_URL || 'http://localhost:8080';
    const fallbackResponse = () => {
        let title = "Custom Offering";
        let type = "Service";
        let price = "50.00";
        if (intent.toLowerCase().includes('guitar')) {
            title = "Beginner Guitar Lesson (1 Hour)";
        } else if (intent.toLowerCase().includes('cake') || intent.toLowerCase().includes('cupcake') || intent.toLowerCase().includes('cookie') || intent.toLowerCase().includes('baking')) {
            title = "Custom Baked Goods";
            type = "Product";
            price = "35.00";
        }
        return NextResponse.json({
            title: title,
            description: `Automatically generated description for: ${intent}. This covers all the essentials needed.`,
            type: type,
            price: price
        });
    };

    try {
        const res = await fetch(`${backendUrl}/api/v1/agents/generate-offering`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': request.headers.get('authorization') || ''
            },
            body: JSON.stringify({ intent })
        });

        if (!res.ok) {
           return fallbackResponse();
        }

        const data = await res.json();
        return NextResponse.json(data);
    } catch (err) {
        console.warn('Backend unavailable, falling back to local generation mock.', err);
        return fallbackResponse();
    }
  } catch (error) {
    return NextResponse.json({ error: 'Failed to process request' }, { status: 500 });
  }
}
