import { NextResponse } from 'next/server';

export const dynamic = 'force-dynamic';

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const tenant = searchParams.get('tenant') || 'my-store';

    // Mocking database fetch. In a real scenario, this would query
    // the core DB for active products, services, and profile settings.
    const mockData = {
        tenant: tenant,
        bio: `Welcome to ${tenant}! Discover our exclusive services and products below.`,
        links: [
            { id: '1', title: 'Book a Consultation', url: `/booking?tenant=${encodeURIComponent(tenant)}` },
            { id: '2', title: 'Shop Products', url: `/checkout?tenant=${encodeURIComponent(tenant)}` },
            { id: '3', title: 'Contact Us', url: `/inbox?tenant=${encodeURIComponent(tenant)}` }
        ]
    };

    return NextResponse.json(mockData);
  } catch (e: any) {
    console.error(`Link-in-bio generation failed: ${e.message}`);
    return NextResponse.json({ error: 'Failed to generate link-in-bio content' }, { status: 500 });
  }
}
