import { NextResponse } from 'next/server';

export const mockArticles = [
  {
    id: "getting-started",
    category: "Getting Started",
    title: "Welcome to OneHumanCorp",
    content: "Setting up your store is quick and easy. First, connect your bank account. Next, add your products. Finally, customize your store's look and feel. If you get stuck, click the '?' button anywhere.",
    readTime: "2 min"
  },
  {
    id: "payments",
    category: "Payments",
    title: "Accepting Your First Payment",
    content: "To accept a payment, go to the Payments tab. Enter the amount and the customer's email. We'll send them a secure link to pay. You'll get an alert when the money is in your account.",
    readTime: "3 min"
  },
  {
    id: "ai-agents",
    category: "AI Agents",
    title: "Activate Your AI Support Agent",
    content: "Your AI agent can answer customer questions 24/7. To turn it on, go to AI Agents and click 'Activate'. It uses the information from your website to know how to answer. You can see all its conversations in the inbox.",
    readTime: "4 min"
  },
  {
    id: "marketing",
    category: "Marketing",
    title: "Sending an Email Campaign",
    content: "Keep your customers coming back with emails. Go to Marketing, select 'New Campaign', and pick a template. Write your message, choose who to send it to, and click send. We'll show you how many people open it.",
    readTime: "5 min"
  },
  {
    id: "account-billing",
    category: "Account & Billing",
    title: "Managing Your Subscription",
    content: "You can change your plan at any time in the Account settings. We only charge you for what you use. If you need a copy of an invoice, you can download it from the Billing History page.",
    readTime: "1 min"
  }
];

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const category = searchParams.get('category');

  let results = mockArticles;
  if (category) {
    results = results.filter(a => a.category.toLowerCase() === category.toLowerCase());
  }

  return NextResponse.json(results);
}
