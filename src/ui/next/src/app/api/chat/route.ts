import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { message } = await request.json();

    // Simple mock logic for routing to specialized Help Agent
    let reply = "I'm your Help Agent! ";
    let link = "";

    if (message.toLowerCase().includes("pay") || message.toLowerCase().includes("bank")) {
      reply += "It sounds like you need help with payments or banking. Check out our Payments guide to learn how to connect your bank and get paid.";
      link = "/help/payments";
    } else if (message.toLowerCase().includes("agent") || message.toLowerCase().includes("ai")) {
      reply += "I can help with that! AI Agents are digital employees that work 24/7. Learn how to hire and manage them in our AI Agents guide.";
      link = "/help/ai-agents";
    } else if (message.toLowerCase().includes("market") || message.toLowerCase().includes("grow")) {
      reply += "Ready to grow your business? Our Marketing guide covers social media, email, and using AI for marketing.";
      link = "/help/marketing";
    } else if (message.toLowerCase().includes("store") || message.toLowerCase().includes("product")) {
      reply += "Need help managing your store or products? Our My Store guide has everything you need to know.";
      link = "/help/my-store";
    } else if (message.toLowerCase().includes("account") || message.toLowerCase().includes("billing") || message.toLowerCase().includes("premium")) {
      reply += "For questions about your subscription, receipts, or upgrading to Premium, please review the Account & Billing guide.";
      link = "/help/account-billing";
    } else {
      reply += "Check out our Getting Started guide for a quick overview of how to set up your store.";
      link = "/help/getting-started";
    }

    const fullReply = `${reply} <br/><br/><a href="${link}" class="text-blue-600 font-bold hover:underline">Read the full article →</a>`;
    return NextResponse.json({ reply: fullReply });
  } catch (error) {
    return NextResponse.json({ reply: "I'm your AI assistant! Since you're asking about store setup, check out our Getting Started guide. <br/><br/><a href=\"/help/getting-started\" class=\"text-blue-600 font-bold hover:underline\">Read the full article →</a>" });
  }
}
