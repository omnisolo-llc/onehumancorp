import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    const body = await request.json();
    const incomingMessage = body.message || '';

    // Simulate query of business context and generating an AI auto-reply
    await new Promise(resolve => setTimeout(resolve, 1500));

    // Agentic Auto-Responder Logic
    let draft_reply = "Thank you for reaching out. Our team will get back to you shortly.";
    let status = "pending";

    const msgLower = incomingMessage.toLowerCase();

    if (msgLower.includes('vegan')) {
        draft_reply = "Hi there! Yes, we absolutely make vegan cakes. They require 48 hours notice. Would you like to see our vegan flavor menu?";
    } else if (msgLower.includes('open') || msgLower.includes('hours')) {
        draft_reply = "We are open Monday through Saturday from 8 AM to 6 PM. Can we help you with anything else?";
    } else if (msgLower.includes('price') || msgLower.includes('cost')) {
        draft_reply = "Our custom cakes start at $50. The final price depends on the design and size. Would you like to schedule a consultation?";
    }

    return NextResponse.json({
      success: true,
      message_id: `msg_${Date.now()}`,
      draft_reply,
      status
    });
}
