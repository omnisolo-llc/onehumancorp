import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json([
    { title: 'Getting Started', desc: 'Welcome to One Human Corp. Learn the basics.', link: '/help/getting-started' },
    { title: 'My Store', desc: 'How to add products, photos, and descriptions.', link: '/help/my-store' },
    { title: 'Payments', desc: 'How to get paid and manage your money.', link: '/help/payments' },
    { title: 'AI Agents', desc: 'Hire AI to answer emails and do the heavy lifting.', link: '/help/ai-agents' },
    { title: 'Marketing', desc: 'Let AI write your social media posts.', link: '/help/marketing' },
    { title: 'Account & Billing', desc: 'Manage your plan and invoices.', link: '/help/account-billing' }
  ]);
}
