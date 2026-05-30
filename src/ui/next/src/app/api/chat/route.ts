import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  let body;
  try {
    body = await req.json();
  } catch (e) {
    return NextResponse.json({ error: 'Invalid JSON body' }, { status: 400 });
  }

  const { message } = body;

  if (!message || typeof message !== 'string' || message.trim() === '') {
    return NextResponse.json({ error: 'message is required' }, { status: 400 });
  }

  if (message.length > 1000) {
    return NextResponse.json({ error: 'message too long' }, { status: 413 });
  }

  const msg = message.toLowerCase();

  let reply = "I am your AI Help Agent! I specialize in answering questions about OHC features and helping you grow your small business.";
  let link = { url: "/help", title: "Visit Help Center →" };

  if (msg.includes("start") || msg.includes("how do i") || msg.includes("setup")) {
    reply = "Setting up is easy! You just need to describe your business in the Builder, pick a vibe, and hit Launch. Check out our step-by-step guide.";
    link = { url: "/help/getting-started", title: "Read Getting Started Guide →" };
  } else if (msg.includes("pay") || msg.includes("stripe") || msg.includes("money") || msg.includes("deposit")) {
    reply = "To get paid, you need to connect your Stripe account. We support credit cards, Apple Pay, and more. Read how to set it up.";
    link = { url: "/help/payments", title: "Learn about Payments →" };
  } else if (msg.includes("agent") || msg.includes("helper") || msg.includes("workforce") || msg.includes("swarm")) {
    reply = "Your AI team includes The Ambassador for support, The Manager for operations, and The Promoter for marketing. Learn how they work together.";
    link = { url: "/help/ai-agents", title: "Meet your AI Team →" };
  } else if (msg.includes("store") || msg.includes("product") || msg.includes("item") || msg.includes("inventory")) {
    reply = "You can manage your items, track stock levels, and change your store's design anytime from the Dashboard and Builder.";
    link = { url: "/help/my-store", title: "Manage your Store →" };
  } else if (msg.includes("customer") || msg.includes("marketing") || msg.includes("refer") || msg.includes("social")) {
    reply = "We help you find customers through social sharing, SEO, and our powerful referral program. Learn more here.";
    link = { url: "/help/marketing", title: "Marketing Guide →" };
  } else if (msg.includes("billing") || msg.includes("plan") || msg.includes("cancel") || msg.includes("upgrade")) {
    reply = "You can manage your subscription, view your plan, and upgrade to Pro from the Account & Billing section.";
    link = { url: "/help/account-billing", title: "View Billing Info →" };
  }

  return NextResponse.json({ reply, link });
}
