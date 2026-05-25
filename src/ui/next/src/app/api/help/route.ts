import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json([
    { title: "Getting Started", desc: "Learn to set up your store and get paid.", link: "/help/getting-started" },
    { title: "My Store", desc: "Add products, track inventory, and design your store.", link: "/help/my-store" },
    { title: "Getting Paid", desc: "Set up payouts, view deposits, and manage taxes.", link: "/help/payments" },
    { title: "Your AI Helpers", desc: "Hire AI helpers and give them tasks.", link: "/help/ai-agents" },
    { title: "Finding Customers", desc: "Send emails and grow your business.", link: "/help/marketing" },
    { title: "Account & Billing", desc: "View bills, manage your plan, and invite your team.", link: "/help/account-billing" }
  ]);
}
