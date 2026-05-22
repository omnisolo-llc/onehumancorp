import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  return NextResponse.json({
    reply: "I'm your AI assistant! Since you're asking about store setup, check out our Getting Started guide.",
    link: {
      url: "/help",
      title: "Read the full article →"
    }
  });
}
