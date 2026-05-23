import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const { message } = await req.json();
  return NextResponse.json({
    reply: "I am your AI Help Agent! I specialize in answering questions about OHC features. For store setup, check out the Getting Started guide.",
    link: { url: "/help", title: "Read the full article →" }
  });
}
