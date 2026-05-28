import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const { message } = await req.json();
  return NextResponse.json({
    reply: "I am your AI Help Agent! I specialize in answering questions about OHC features and helping you grow your small business. For example, I can help you set up a custom domain for your store. Check out our Help Center.",
    link: { url: "/help", title: "Read the full article →" }
  });
}
