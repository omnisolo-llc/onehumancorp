import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  let message = "";
  try {
    const body = await req.json();
    message = body.message || "";
  } catch (e) {
    message = "";
  }
  const lowerMsg = message.toLowerCase();

  let reply = "I am your AI Help Agent! I specialize in answering questions about OHC features and helping you grow your small business. Check out our Getting Started guide.";
  let link = { url: "/help/getting-started", title: "Getting Started →" };

  if (lowerMsg.includes('payment') || lowerMsg.includes('get paid') || lowerMsg.includes('stripe') || lowerMsg.includes('tax')) {
    reply = "It's easy to get paid! We help handle simple taxes at checkout. A small fee is taken out of each sale to cover the cost of securely moving the money from the customer's card to your bank.";
    link = { url: "/help/payments", title: "Read the full article →" };
  } else if (lowerMsg.includes('store') || lowerMsg.includes('product') || lowerMsg.includes('stock') || lowerMsg.includes('inventory')) {
    reply = "To add a new item, go to the products page and click 'Add Product'. You can upload a picture, type in a name and description, and set the price. Our AI can even help you write a catchy description!";
    link = { url: "/help/my-store", title: "Read the full article →" };
  } else if (lowerMsg.includes('ai') || lowerMsg.includes('helper') || lowerMsg.includes('agent') || lowerMsg.includes('hire')) {
    reply = "Running a business takes a lot of work. That's why we give you AI helpers—smart computer programs that can do tasks for you! You can hire them from the AI Departments page.";
    link = { url: "/help/ai-agents", title: "Read the full article →" };
  } else if (lowerMsg.includes('marketing') || lowerMsg.includes('email') || lowerMsg.includes('customer') || lowerMsg.includes('grow')) {
    reply = "You can send emails to people who have bought from you before or signed up on your store. Our AI can even help you write the emails!";
    link = { url: "/help/marketing", title: "Read the full article →" };
  } else if (lowerMsg.includes('bill') || lowerMsg.includes('plan') || lowerMsg.includes('account') || lowerMsg.includes('team')) {
    reply = "You can manage your subscription plan, view your past bills, and invite people to help run your business from the Account settings.";
    link = { url: "/help/account-billing", title: "Read the full article →" };
  } else if (lowerMsg.includes('start') || lowerMsg.includes('setup') || lowerMsg.includes('begin')) {
    reply = "Welcome to OneHumanCorp! Setting up your store is quick and easy. Our system helps you get everything ready to sell online.";
    link = { url: "/help/getting-started", title: "Read the full article →" };
  }

  return NextResponse.json({ reply, link });
}
