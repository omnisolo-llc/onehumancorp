import { NextResponse } from 'next/server';

export async function GET() {
  const videos = [
    { id: 1, title: "Set up your store", duration: "1:15", portrait: true },
    { id: 2, title: "Add your first product", duration: "1:20", portrait: true },
    { id: 3, title: "Connect your bank account", duration: "1:10", portrait: true },
    { id: 4, title: "Hire your first AI agent", duration: "1:25", portrait: true },
    { id: 5, title: "Design your storefront", duration: "1:05", portrait: true },
    { id: 6, title: "Manage your orders", duration: "0:55", portrait: true },
    { id: 7, title: "Promote on social media", duration: "1:18", portrait: true },
    { id: 8, title: "Launch your first campaign", duration: "1:22", portrait: true },
    { id: 9, title: "Read your sales reports", duration: "1:12", portrait: true },
    { id: 10, title: "Change your monthly plan", duration: "0:45", portrait: true }
  ];

  return NextResponse.json(videos);
}
