import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { message } = await request.json();
    const msgLower = (message || "").toLowerCase();

    let reply = "I'm your AI assistant! How can I help you grow your business today?";

    if (msgLower.includes("setup") || msgLower.includes("store") || msgLower.includes("build")) {
      reply = "To get started, simply head over to the store builder and describe what you want to sell. Our AI will automatically construct a ready-to-launch store for you based on your description.<br/><br/><a href=\"/help\" class=\"text-blue-600 font-bold hover:underline\">Read the full article →</a>";
    } else if (msgLower.includes("pay") || msgLower.includes("bank")) {
      reply = "You can manage your payments securely by connecting your bank account in the Payments section.<br/><br/><a href=\"/help\" class=\"text-blue-600 font-bold hover:underline\">Read the full article →</a>";
    } else if (msgLower.includes("help") || msgLower.includes("guide")) {
      reply = "Since you're asking about store setup, check out our Getting Started guide.<br/><br/><a href=\"/help\" class=\"text-blue-600 font-bold hover:underline\">Read the full article →</a>";
    }

    return NextResponse.json({ reply });
  } catch (err) {
    return NextResponse.json({ reply: "I'm your AI assistant! Since you're asking about store setup, check out our Getting Started guide. <br/><br/><a href=\"/help\" class=\"text-blue-600 font-bold hover:underline\">Read the full article →</a>" });
  }
}
