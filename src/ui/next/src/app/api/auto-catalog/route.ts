import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    // Simulate AI extraction delay
    await new Promise(resolve => setTimeout(resolve, 1500));

    return NextResponse.json({
        success: true,
        data: {
            title: 'Artisan Vanilla Bean Cupcake',
            price: '4.99',
            category: 'Baked Goods',
            description: 'Hand-crafted vanilla bean cupcake topped with Madagascar vanilla buttercream frosting. Made fresh daily with organic ingredients.'
        }
    });
}
