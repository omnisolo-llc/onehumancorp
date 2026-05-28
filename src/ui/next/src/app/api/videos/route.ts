import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json([
    { id: 1, title: "Setting up your store in 60 seconds", duration: "1:00" },
    { id: 2, title: "How to add products", duration: "0:45" },
    { id: 3, title: "Understanding your first payout", duration: "1:29" },
    { id: 4, title: "Hiring your first AI helper", duration: "1:05" },
    { id: 5, title: "Connecting a custom domain", duration: "1:20" },
    { id: 6, title: "Creating a discount code", duration: "0:50" },
    { id: 7, title: "Sending your first email campaign", duration: "1:10" },
    { id: 8, title: "Customizing your storefront design", duration: "1:25" },
    { id: 9, title: "Managing customer refunds", duration: "0:55" },
    { id: 10, title: "Inviting team members", duration: "0:40" }
  ]);
}
