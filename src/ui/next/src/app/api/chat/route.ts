import { NextResponse } from 'next/server';

const MAX_MESSAGE_LENGTH = 1000;

const ARTICLES = [
  { title: "Getting Started", desc: "Learn how to easily set up your store and accept your first payment.", link: "/help/getting-started", keywords: ["start", "setup", "begin"] },
  { title: "My Store", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/my-store", keywords: ["store", "product", "stock", "look", "inventory"] },
  { title: "Getting Paid", desc: "Set up how you get paid, view deposits, and handle simple taxes.", link: "/help/payments", keywords: ["pay", "money", "stripe", "bank", "deposit", "tax"] },
  { title: "Your AI Helpers", desc: "Learn how to hire AI helpers and give them tasks to do.", link: "/help/ai-agents", keywords: ["ai", "agent", "helper", "hire", "task"] },
  { title: "Finding Customers", desc: "Send emails to customers and grow your business easily.", link: "/help/marketing", keywords: ["customer", "email", "market", "grow", "sale", "promo"] },
  { title: "Account & Billing", desc: "View your bills, manage your plan, and invite team members.", link: "/help/account-billing", keywords: ["account", "bill", "plan", "team", "invite"] }
];

export async function POST(req: Request) {
  let body: unknown;
  try {
    body = await req.json();
  } catch {
    return NextResponse.json({ error: "Invalid JSON body" }, { status: 400 });
  }

  const message = body && typeof body === 'object' && 'message' in body
    ? (body as { message?: unknown }).message
    : undefined;

  if (typeof message !== 'string' || !message.trim()) {
    return NextResponse.json({ error: "message is required" }, { status: 400 });
  }

  if (message.trim().length > MAX_MESSAGE_LENGTH) {
    return NextResponse.json({ error: `message must be ${MAX_MESSAGE_LENGTH} characters or fewer` }, { status: 413 });
  }

  const query = message.toLowerCase();

  let bestMatch = ARTICLES[0]; // Default to Getting Started
  let maxMatches = 0;

  for (const article of ARTICLES) {
    let matches = 0;
    if (query.includes(article.title.toLowerCase())) matches += 2;
    for (const keyword of article.keywords) {
      if (query.includes(keyword)) matches += 1;
    }
    if (matches > maxMatches) {
      maxMatches = matches;
      bestMatch = article;
    }
  }

  if (maxMatches === 0) {
    return NextResponse.json({
      reply: "I am your AI Help Agent! I specialize in answering questions about OHC features and helping you grow your small business. Check out our Help Center.",
      link: { url: "/help", title: "Go to Help Center →" }
    });
  }

  return NextResponse.json({
    reply: `I can help with that! It sounds like you are asking about ${bestMatch.title}. ${bestMatch.desc}`,
    link: { url: bestMatch.link, title: `Read the full article →` }
  });
}
