import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    const body = await request.json();
    const incomingMessage = body.message || '';

    // Simulate query of business context and generating reply
    await new Promise(resolve => setTimeout(resolve, 1500));

    let reply = "Thanks for your message! Our AI agent will assist you shortly.";

    if (incomingMessage.toLowerCase().includes('open')) {
        reply = "Hi! Yes, we are open until 6 PM today and we currently have 12 Vanilla Cupcakes left. Shall I set one aside for you?";
    } else if (incomingMessage.toLowerCase().includes('price')) {
        reply = "Hi! Our Vanilla Bean Cupcakes are $4.99 each. Would you like to place an order?";
    }

    return NextResponse.json({ reply });
}
