import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const { message } = await req.json();
  const lowerMsg = message.toLowerCase();

  // Basic search logic using help articles content
  const articles = [
    { id: "getting-started", title: "Getting Started", desc: "Learn how to easily set up your store and accept your first payment.", link: "/help/getting-started" },
    { id: "my-store", title: "My Store", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/my-store" },
    { id: "payments", title: "Getting Paid", desc: "Set up how you get paid, view deposits, and handle simple taxes.", link: "/help/payments" },
    { id: "ai-agents", title: "Your AI Helpers", desc: "Learn how to hire AI helpers and give them tasks to do.", link: "/help/ai-agents" },
    { id: "marketing", title: "Finding Customers", desc: "Send emails to customers and grow your business easily.", link: "/help/marketing" },
    { id: "account-billing", title: "Account & Billing", desc: "View your bills, manage your plan, and invite team members.", link: "/help/account-billing" }
  ];

  let bestMatch = null;
  for (const article of articles) {
    if (lowerMsg.includes(article.id.replace('-', ' ')) || lowerMsg.includes(article.title.toLowerCase()) || lowerMsg.includes(article.desc.split(',')[0].toLowerCase())) {
      bestMatch = article;
      break;
    }
  }

  // Custom keywords fallback
  if (!bestMatch) {
      if (lowerMsg.includes('pay') || lowerMsg.includes('money') || lowerMsg.includes('stripe')) {
          bestMatch = articles[2]; // payments
      } else if (lowerMsg.includes('add') || lowerMsg.includes('product') || lowerMsg.includes('stock')) {
          bestMatch = articles[1]; // my store
      } else if (lowerMsg.includes('setup') || lowerMsg.includes('start')) {
          bestMatch = articles[0]; // getting started
      } else if (lowerMsg.includes('ai') || lowerMsg.includes('bot') || lowerMsg.includes('help')) {
          bestMatch = articles[3]; // ai-agents
      } else if (lowerMsg.includes('email') || lowerMsg.includes('customer') || lowerMsg.includes('grow')) {
          bestMatch = articles[4]; // marketing
      } else if (lowerMsg.includes('bill') || lowerMsg.includes('plan') || lowerMsg.includes('account')) {
          bestMatch = articles[5]; // account & billing
      }
  }

  if (bestMatch) {
    return NextResponse.json({
      reply: `I found a helpful article about "${bestMatch.title}". This should give you exactly what you need!`,
      link: { url: bestMatch.link, title: "Read the full article →" }
    });
  }

  return NextResponse.json({
    reply: "I am your AI Help Agent! I specialize in answering questions about OHC features and helping you grow your small business. Check out our Getting Started guide.",
    link: { url: "/help/getting-started", title: "Read the full article →" }
  });
}
