import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const data = await req.json();

    // In a real implementation this would proxy to the Rust backend
    // Since the API gateway mock logic isn't fully set up for this specific route,
    // we'll simulate a successful save response that acknowledges the new fields.

    return NextResponse.json({
      success: true,
      service: {
        id: 'mock-service-id',
        title: data.title,
        price: data.price,
        autoPricingEnabled: data.autoPricingEnabled || false,
        minPrice: data.minPrice || null,
        maxPrice: data.maxPrice || null
      }
    });
  } catch (error) {
    return NextResponse.json(
      { success: false, error: 'Failed to create service' },
      { status: 500 }
    );
  }
}
