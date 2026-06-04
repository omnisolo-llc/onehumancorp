import { NextResponse } from 'next/server';

const ARTICLES = [
    { title: "Getting Started", desc: "Learn how to easily set up your store and accept your first payment.", link: "/help/getting-started-1" },
  { title: "My Store", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/my-store" },
  { title: "Getting Paid", desc: "Set up how you get paid, view deposits, and handle simple taxes.", link: "/help/payments" },
  { title: "Your AI Helpers", desc: "Learn how to hire AI helpers and give them tasks to do.", link: "/help/ai-agents" },
  { title: "Finding Customers", desc: "Send emails to customers and grow your business easily.", link: "/help/marketing" },
  { title: "Account & Billing", desc: "View your bills, manage your plan, and invite team members.", link: "/help/account-billing" }
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
