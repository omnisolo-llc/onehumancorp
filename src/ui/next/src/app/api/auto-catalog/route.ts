import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    // Simulate processing delay for the AutoDream pipeline
    await new Promise(resolve => setTimeout(resolve, 2000));

    return NextResponse.json({
        title: "Artisan Vanilla Bean Cupcake",
        description: "A delightful, handcrafted vanilla bean cupcake with a rich, buttery crumb and a swirl of velvety buttercream frosting. Perfect for celebrations or a sweet afternoon treat.",
        price: "4.99",
        category: "Baked Goods"
    });
}
