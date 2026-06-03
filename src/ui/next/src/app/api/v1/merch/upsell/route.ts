import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const body = await req.json();
    const cartItems = body.cart_items || [];
    const tenantId = req.headers.get("x-tenant-id") || "default";

    // Autonomous AI Product Bundling logic
    // Mocking the AI inference logic for demo
    // In a real scenario, this would call the Rust backend which calls Gemini
    const recommendations = [];

    const hasCandle = cartItems.some((item: any) => item.name.toLowerCase().includes('candle'));
    if (hasCandle) {
        recommendations.push({
            id: 'upsell_matches',
            name: 'Premium Matches',
            price: 5.00,
            original_price: 8.00,
            description: 'Perfectly pairs with your candle. Add to bundle for 37% off.',
            image_url: 'https://images.unsplash.com/photo-1599839619722-39751411ea63?w=400&q=80'
        });
    }

    const hasGuitar = cartItems.some((item: any) => item.name.toLowerCase().includes('guitar'));
    if (hasGuitar) {
         recommendations.push({
            id: 'upsell_picks',
            name: 'Custom Guitar Picks (10-pack)',
            price: 4.00,
            original_price: 6.00,
            description: 'Never run out of picks. Add to bundle for 33% off.',
            image_url: 'https://images.unsplash.com/photo-1510914109727-40da820c75b8?w=400&q=80'
        });
    }

    // Default recommendation if no matches
    if (recommendations.length === 0) {
        recommendations.push({
            id: 'upsell_mystery',
            name: 'Mystery Gift Box',
            price: 15.00,
            original_price: 25.00,
            description: 'A surprise bundle of our best sellers. Add to bundle for 40% off.',
            image_url: 'https://images.unsplash.com/photo-1549465220-1a8b9238cd48?w=400&q=80'
        });
    }

    return NextResponse.json({
        success: true,
        recommendations: recommendations
    });

  } catch (error: any) {
    console.error("Upsell inference error:", error);
    return NextResponse.json({ success: false, error: error.message }, { status: 500 });
  }
}
