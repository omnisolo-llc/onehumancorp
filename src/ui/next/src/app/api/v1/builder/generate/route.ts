import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { description } = await request.json();
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';

    // We can proxy to the intake endpoint to simulate AI generation
    const res = await fetch(`${backendUrl}/api/onboarding/intake`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ description })
    });

    if (res.ok) {
        const intakeData = await res.json();
        const data = {
            pages: [
            {
                blocks: [
                {
                    block_type: 'HeroBlock',
                    content: {
                    headline: intakeData.business_name || 'Your Custom Business',
                    subheadline: intakeData.business_type || description.substring(0, 50) + '...',
                    cta_text: 'Book Now'
                    }
                },
                {
                    block_type: 'ProductGridBlock',
                    content: {
                    title: 'Featured Services',
                    items: intakeData.initial_products || [
                        { name: 'Initial Consultation', price: '$50.00' }
                    ]
                    }
                }
                ]
            }
            ]
        };
        return NextResponse.json(data);
    }

    // Fallback stub if backend is down or fails
    const data = {
      pages: [
        {
          blocks: [
            {
              block_type: 'HeroBlock',
              content: {
                headline: 'Your Custom Business',
                subheadline: description.substring(0, 50) + '...',
                cta_text: 'Book Now'
              }
            },
            {
              block_type: 'ProductGridBlock',
              content: {
                title: 'Featured Services',
                items: [
                  { name: 'Initial Consultation', price: '$50.00' }
                ]
              }
            }
          ]
        }
      ]
    };

    return NextResponse.json(data);
  } catch (error) {
    return NextResponse.json({ error: 'Failed to generate' }, { status: 500 });
  }
}
