import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json([
    { title: "Getting Started", desc: "Learn how to easily set up your store and accept your first payment.", link: "/help/article/getting-started" },
    { title: "My Store", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/article/my-store" },
    { title: "Getting Paid", desc: "Set up how you get paid, view deposits, and handle simple taxes.", link: "/help/article/payments" },
    { title: "Your AI Helpers", desc: "Learn how to hire AI helpers and give them tasks to do.", link: "/help/article/ai-agents" },
    { title: "Finding Customers", desc: "Send emails to customers and grow your business easily.", link: "/help/article/marketing" },
    { title: "Account & Billing", desc: "View your bills, manage your plan, and invite team members.", link: "/help/article/account-billing" }
  ]);
}
