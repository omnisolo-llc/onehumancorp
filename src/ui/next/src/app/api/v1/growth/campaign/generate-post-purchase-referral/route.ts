import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { store, customerName } = await req.json();
    const name = customerName || "there";
    const storeName = store || "our store";

    // Mock AI generation delay
    await new Promise((resolve) => setTimeout(resolve, 1500));

    const message = `Hi ${name},\n\nThanks for your recent purchase from ${storeName}! We hope you are absolutely loving it.\n\nWant 20% off your next order? Give your friends 15% off their first purchase. When they buy, you get 20% off!\n\nShare your link now: https://ohc.store/join?ref=customer-${name.toLowerCase()}\n\nWarmly,\nThe ${storeName} Team\n\n⚡ Powered by OHC`;

    return NextResponse.json({ message });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to generate campaign' }, { status: 500 });
  }
}
