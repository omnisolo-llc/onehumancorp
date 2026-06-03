import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { message } = await req.json();

    if (!message) {
      return NextResponse.json({ error: 'Message is required' }, { status: 400 });
    }

    const lowerMsg = message.toLowerCase();

    // Simple intent detection based on the issue description
    let draft_reply = '';
    let escalate = false;

    if (lowerMsg.includes('order') || lowerMsg.includes('where is') || lowerMsg.includes('tracking')) {
      draft_reply = "Hi there! I can help you with that. Could you please provide your order number?";
    } else if (lowerMsg.includes('vegan')) {
      draft_reply = "Yes, we have several vegan options available! You can order them directly from our website or let me know what flavors you are interested in.";
    } else if (lowerMsg.includes('price') || lowerMsg.includes('cost')) {
      draft_reply = "Our prices vary depending on the specific item and customization. You can find a full price list on our website, or let me know exactly what you're looking for!";
    } else if (lowerMsg.includes('open')) {
      draft_reply = "Hi! Yes, we are open until 6 PM today and we currently have 12 Vanilla Cupcakes left. Shall I set one aside for you?";
    } else {
      escalate = true;
      draft_reply = "I'm escalating this to a human team member. They will get back to you shortly!";
    }

    return NextResponse.json({
      success: true,
      draft_reply,
      escalate
    });
  } catch (error) {
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
