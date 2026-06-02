import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    const body = await request.json();
    const incomingMessage = body.message || '';
    const tenantId = body.tenantId || 'default_tenant';

    try {
        // Attempt to call a real backend AI service if configured.
        // For the sake of this implementation, we will use a more robust mock
        // that simulates the 'Ambassador Agent' consulting the Knowledge Base and
        // returning an intelligent response.

        // Simulate query of business context and generating reply
        await new Promise(resolve => setTimeout(resolve, 800));

        let reply = "Thanks for your message! Our AI agent will assist you shortly.";

        const lowerMessage = incomingMessage.toLowerCase();

        if (lowerMessage.includes('open') || lowerMessage.includes('hours')) {
            reply = "Hi! Yes, we are open until 6 PM today and we currently have 12 Vanilla Cupcakes left. Shall I set one aside for you?";
        } else if (lowerMessage.includes('price') || lowerMessage.includes('cost') || lowerMessage.includes('how much')) {
            reply = "Hi! Our Vanilla Bean Cupcakes are $4.99 each. Would you like to place an order?";
        } else if (lowerMessage.includes('vegan') || lowerMessage.includes('allergy')) {
             reply = "Yes, we have several vegan birthday cake options available! You can order them directly from our website or let me know what flavors you are interested in.";
        } else if (lowerMessage.includes('address') || lowerMessage.includes('location')) {
             reply = "We are located at 123 Bakery Lane, Sweet City. You can find directions on our website.";
        }

        return NextResponse.json({
            reply: reply,
            agent: 'The Ambassador',
            confidence: 0.95,
            requiresHumanEscalation: false,
        });

    } catch (error) {
        console.error("Error generating Ambassador reply:", error);
        return NextResponse.json({
             reply: "I'm having trouble connecting right now, but a team member will get back to you soon.",
             requiresHumanEscalation: true
        }, { status: 500 });
    }
}
