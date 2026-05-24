import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const { message } = await req.json();
  const msg = message.toLowerCase();

  let reply = "I am your AI Help Agent! I specialize in answering questions about OHC features and helping you grow your small business.";
  let link = { url: "/help", title: "View Help Center →" };

  if (msg.includes("payment") || msg.includes("paid") || msg.includes("money")) {
    reply = "You can set up payments by connecting your bank account through Stripe. This allows you to safely accept money from customers all over the world.";
    link = { url: "/help/payments", title: "How to get paid →" };
  } else if (msg.includes("store") || msg.includes("website") || msg.includes("products")) {
    reply = "Your store is where customers see your products. You can change how it looks, add new items, and update your business info anytime.";
    link = { url: "/help/my-store", title: "Managing your store →" };
  } else if (msg.includes("agent") || msg.includes("helper") || msg.includes("swarm")) {
    reply = "AI agents are your digital workforce. They can help with customer service, marketing, and even managing your inventory automatically.";
    link = { url: "/help/ai-agents", title: "Working with AI helpers →" };
  } else if (msg.includes("start") || msg.includes("setup") || msg.includes("how")) {
    reply = "Welcome! The best way to start is our Getting Started guide, which walks you through setting up your store in under 5 minutes.";
    link = { url: "/help/getting-started", title: "Start your journey →" };
  }

  return NextResponse.json({ reply, link });
}
