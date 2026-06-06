import { NextResponse } from 'next/server';

const ARTICLES = [
  { title: "Getting Started", desc: "Learn how Maya the baker goes from cake idea to a live store in under 10 minutes.", link: "/help/getting-started-1" },
  { title: "My Store", desc: "Add products like Carlos the handyman or Fatima the food cart owner. Track stock easily.", link: "/help/my-store" },
  { title: "Payments", desc: "How Priya uses Stripe Terminal for in-person tap-to-pay and online checkouts.", link: "/help/payments" },
  { title: "Your AI Helpers", desc: "Meet your workforce. Agents that reply to DMs, optimize SEO, and manage Maya's inventory.", link: "/help/ai-agents" },
  { title: "Finding Customers", desc: "Automated Instagram posts and Google SEO so customers find your local business.", link: "/help/marketing" },
  { title: "Account & Billing", desc: "Simple plan management and team invites for your growing business.", link: "/help/account-billing" }
];

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const q = searchParams.get('q')?.toLowerCase() || '';

  const results = ARTICLES.filter(a =>
    a.title.toLowerCase().includes(q) ||
    a.desc.toLowerCase().includes(q)
  );

  return NextResponse.json(results);
}
