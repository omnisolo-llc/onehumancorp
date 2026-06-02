import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();

    if (!body || !body.description) {
        return NextResponse.json({ error: 'Description is required' }, { status: 400 });
    }

    // Since this is the Next.js UI prototype layer, we mock the delay for instant generation
    // In a real environment, we would forward to backendUrl/api/onboarding/instant-build
    await new Promise((resolve) => setTimeout(resolve, 2000));

    return NextResponse.json({ message: "Storefront generated successfully", liveUrl: "my-business.ohc.store" });
  } catch (e) {
    return NextResponse.json({ error: 'Failed to generate storefront' }, { status: 500 });
  }
}
