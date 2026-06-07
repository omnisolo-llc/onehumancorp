import { NextResponse } from 'next/server';

const promotions = [
  "Hey there! 🎉 We're running an exclusive flash sale this weekend. Grab your favorite items before they're gone! Shop now and get 10% off: https://ohc.store/shop/{tenant}",
  "🚀 Just dropped! Check out our latest arrivals and get a special 15% discount on your first order. Use code NEW15 at checkout! https://ohc.store/shop/{tenant}",
  "✨ Special offer just for you! Buy one, get one 50% off on all accessories this week. Don't miss out! Shop now: https://ohc.store/shop/{tenant}",
  "🔥 Limited time offer: Free shipping on all orders over $50! Stock up on your essentials today. https://ohc.store/shop/{tenant}",
  "🎁 Treat yourself! Use code TREAT20 for 20% off your entire purchase today. Shop the collection: https://ohc.store/shop/{tenant}",
];

export async function POST(request: Request) {
  try {
    const { tenant } = await request.json();
    const tenantName = tenant || 'my-store';

    // Pick a random promotion
    const randomPromo = promotions[Math.floor(Math.random() * promotions.length)];
    const message = randomPromo.replace('{tenant}', tenantName);

    return NextResponse.json({ message });
  } catch (error) {
    console.error("Error generating promotion:", error);
    return NextResponse.json(
      { error: "Failed to generate promotion" },
      { status: 500 }
    );
  }
}
