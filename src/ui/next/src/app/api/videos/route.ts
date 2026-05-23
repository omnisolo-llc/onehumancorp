import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json([
    { id: 1, title: "How to set up your first store", duration: "1:20" },
    { id: 2, title: "Connecting a custom domain", duration: "0:45" },
    { id: 3, title: "Accepting payments with Stripe", duration: "1:10" },
    { id: 4, title: "Hiring your first AI Agent", duration: "1:05" },
    { id: 5, title: "Managing your product catalog", duration: "0:55" },
    { id: 6, title: "Setting up automated marketing", duration: "1:15" },
    { id: 7, title: "Understanding your store analytics", duration: "0:50" },
    { id: 8, title: "Processing your first order", duration: "1:00" },
    { id: 9, title: "Customizing your store design", duration: "1:25" },
    { id: 10, title: "Inviting team members", duration: "0:40" }
  ]);
}
