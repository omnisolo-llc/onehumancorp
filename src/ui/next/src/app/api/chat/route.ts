import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const { message } = await req.json();
  const lowerMsg = message.toLowerCase();

  let reply = "I am your AI Help Agent! I specialize in answering questions about OHC features and helping you grow your small business.";
  let link = null;

  if (lowerMsg.includes('start') || lowerMsg.includes('setup') || lowerMsg.includes('begin')) {
    reply = "Setting up your store is quick and easy! First, tell us about your business, then let our AI generate your storefront, and finally, launch it to the world. For full details, check out our Getting Started guide.";
    link = { url: "/help/getting-started", title: "Read the full article →" };
  } else if (lowerMsg.includes('product') || lowerMsg.includes('inventory') || lowerMsg.includes('stock')) {
    reply = "You can easily add new products, track inventory, and write catchy descriptions using AI. To add an item, go to the products page and click 'Add Product'.";
    link = { url: "/help/my-store", title: "Read the full article →" };
  } else if (lowerMsg.includes('pay') || lowerMsg.includes('money') || lowerMsg.includes('tax')) {
    reply = "We help you get paid securely! A small fee is taken from each sale to process the transaction. You can view your deposits and handle simple taxes directly from your dashboard.";
    link = { url: "/help/payments", title: "Read the full article →" };
  } else if (lowerMsg.includes('agent') || lowerMsg.includes('helper') || lowerMsg.includes('ai')) {
    reply = "AI helpers are like a real team! Go to the AI Departments page to hire specialized agents (like marketers or accountants) and assign them tasks in plain English.";
    link = { url: "/help/ai-agents", title: "Read the full article →" };
  } else if (lowerMsg.includes('customer') || lowerMsg.includes('market') || lowerMsg.includes('email') || lowerMsg.includes('promo')) {
    reply = "Grow your business by sending emails, running promos, and sharing your store link. Our AI can even help you write your marketing emails!";
    link = { url: "/help/marketing", title: "Read the full article →" };
  } else if (lowerMsg.includes('bill') || lowerMsg.includes('plan') || lowerMsg.includes('team') || lowerMsg.includes('invite')) {
    reply = "You can manage your subscription plan, view past bills, and invite team members from the Billing page.";
    link = { url: "/help/account-billing", title: "Read the full article →" };
  } else {
    reply = "I am your AI Help Agent! I specialize in answering questions about OHC features and helping you grow your small business. Check out our Help Center for guides and tutorials.";
    link = { url: "/help", title: "Visit Help Center →" };
  }

  return NextResponse.json({ reply, link });
}
