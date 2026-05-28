import { NextResponse } from 'next/server';

const helpArticles = [
  { keywords: ["store", "product", "stock", "look", "design"], desc: "Learn how to easily add products, track stock, and change your store design.", link: "/help/my-store" },
  { keywords: ["payment", "pay", "bank", "stripe", "deposit", "tax"], desc: "Set up how you get paid, view deposits, and handle simple taxes.", link: "/help/payments" },
  { keywords: ["agent", "ai", "helper", "task", "approve"], desc: "Learn how to hire AI helpers and give them tasks to do.", link: "/help/ai-agents" },
  { keywords: ["email", "marketing", "customer", "promo", "sale", "share"], desc: "Send emails to customers and grow your business easily.", link: "/help/marketing" },
  { keywords: ["account", "bill", "plan", "invite", "team"], desc: "View your bills, manage your plan, and invite team members.", link: "/help/account-billing" },
  { keywords: ["api", "integration", "advanced"], desc: "Interactive API reference for advanced integrations.", link: "/api-docs" },
];

export async function POST(req: Request) {
  const { message } = await req.json();
  const query = message.toLowerCase();

  let reply = "I am your AI Help Agent! I specialize in answering questions about OHC features and helping you grow your small business. Check out our Getting Started guide.";
  let link_url = "/help/getting-started";
  let link_title = "Read the full article →";

  for (const article of helpArticles) {
    if (article.keywords.some(kw => query.includes(kw))) {
      reply = `Based on our help center: ${article.desc}`;
      link_url = article.link;
      break;
    }
  }

  return NextResponse.json({
    reply: reply,
    link: { url: link_url, title: link_title }
  });
}
