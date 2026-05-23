import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json([
    { title: "Getting Started", desc: "Learn how to set up your store and accept your first payment.", link: "/help/getting-started" },
    { title: "My Store", desc: "Manage your products, inventory, and storefront design.", link: "/help/my-store" },
    { title: "Payments", desc: "Configure payment gateways, payouts, and taxes.", link: "/help/payments" },
    { title: "AI Agents", desc: "Learn how to hire, train, and manage your AI workforce.", link: "/help/ai-agents" },
    { title: "Marketing", desc: "Run campaigns, send emails, and grow your audience.", link: "/help/marketing" },
    { title: "Account & Billing", desc: "Manage your subscription, invoices, and team members.", link: "/help/account-billing" }
  ]);
}
