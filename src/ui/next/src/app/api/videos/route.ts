import { NextResponse } from 'next/server';

export async function GET() {
  const videos = [
    { id: 1, title: "Set up your store", duration: "1m 15s" },
    { id: 2, title: "Add your first product", duration: "1m 20s" },
    { id: 3, title: "Accepting payments", duration: "1m 10s" },
    { id: 4, title: "Activate your AI Support Agent", duration: "1m 25s" },
    { id: 5, title: "Design your storefront", duration: "1m 05s" },
    { id: 6, title: "Manage your inventory", duration: "0m 55s" },
    { id: 7, title: "Configure shipping rates", duration: "1m 18s" },
    { id: 8, title: "Launch marketing campaigns", duration: "1m 22s" },
    { id: 9, title: "Analyze your sales", duration: "1m 12s" },
    { id: 10, title: "Upgrade to premium", duration: "0m 45s" }
  ];

  return NextResponse.json(videos);
}
