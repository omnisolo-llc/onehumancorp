import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { message_context } = await req.json();

    if (!message_context) {
      return NextResponse.json({ error: 'Message context is required' }, { status: 400 });
    }

    // Simple mock logic based on the incoming message context
    const lowerContext = message_context.toLowerCase();
    let draft = "Thank you for reaching out! How can I help you today?";

    if (lowerContext.includes('vegan')) {
      draft = "Yes, we have several vegan birthday cake options available! You can order them directly from our website or let me know what flavors you are interested in.";
    } else if (lowerContext.includes('shipped') || lowerContext.includes('order')) {
      draft = "Your order is currently being prepared and will be shipped within 24 hours. You will receive a tracking link shortly.";
    } else if (lowerContext.includes('address') || lowerContext.includes('delivery')) {
      draft = "Certainly! Please provide your new delivery address, and we will update your order right away.";
    }

    return NextResponse.json({ draft });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to generate draft' }, { status: 500 });
  }
}
