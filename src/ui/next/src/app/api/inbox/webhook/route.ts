import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    const body = await request.json();
    const incomingMessage = body.message || '';

    // Simulate query of business context and generating reply
    await new Promise(resolve => setTimeout(resolve, 1500));

    let reply = "Thanks for your message! Our AI agent will assist you shortly.";
    let escalateToHuman = false;

    if (incomingMessage.toLowerCase().includes('open')) {
        reply = "Hi! Yes, we are open until 6 PM today and we currently have 12 Vanilla Cupcakes left. Shall I set one aside for you?";
    } else if (incomingMessage.toLowerCase().includes('price')) {
        reply = "Hi! Our Vanilla Bean Cupcakes are $4.99 each. Would you like to place an order?";
    } else if (incomingMessage.toLowerCase().includes('where is my order')) {
        reply = "I'm sorry, I couldn't find your order details. I'll pass this to our human team to assist you.";
        escalateToHuman = true;
    } else if (incomingMessage.toLowerCase().includes('vegan')) {
        reply = "Yes! We offer a variety of vegan options, including our popular Vegan Chocolate Cake and Vegan Berry Muffins. Let me know what you'd like to order!";
    }

    return NextResponse.json({ reply, escalateToHuman });
}
