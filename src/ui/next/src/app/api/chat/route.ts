import { NextResponse } from 'next/server';

const KNOWLEDGE_BASE = [
  {
    topic: "Getting Started",
    keywords: ["setup", "start", "build", "create", "onboarding"],
    reply: "Setting up your store is quick and easy! Just describe your business to our AI agents, and they will build a ready-to-launch store for you. You don't need any technical skills.",
    link: { url: "/help", title: "Read the 'Getting Started' guide →" }
  },
  {
    topic: "Payments",
    keywords: ["payment", "money", "paid", "stripe", "bank", "sales"],
    reply: "To get paid, you need to connect your bank account using our secure Stripe integration. This lets you accept credit cards from customers all over the world.",
    link: { url: "/help", title: "How to accept your first payment →" }
  },
  {
    topic: "AI Agents",
    keywords: ["agent", "worker", "team", "swarm", "automated", "hiring"],
    reply: "Your AI workforce consists of specialized agents like 'The Manager' and 'The Promoter'. They work 24/7 in the background to handle marketing, customer support, and more.",
    link: { url: "/help", title: "Learn about your AI workforce →" }
  },
  {
    topic: "Marketing",
    keywords: ["marketing", "customer", "sell", "promote", "social", "traffic"],
    reply: "Our marketing tools help you find new customers automatically. You can sync with social media, send email updates, and even improve how search engines find your store.",
    link: { url: "/help", title: "Grow your audience with AI Marketing →" }
  },
  {
    topic: "Account & Billing",
    keywords: ["bill", "plan", "price", "subscription", "upgrade", "invoice"],
    reply: "You can manage your subscription and see all your invoices in the Account & Billing section. We offer plain-language pricing with no hidden fees.",
    link: { url: "/help", title: "Manage your account & billing →" }
  }
];

export async function POST(request: Request) {
  const { message } = await request.json();
  const lowerMessage = message.toLowerCase();

  const match = KNOWLEDGE_BASE.find(entry =>
    entry.keywords.some(keyword => lowerMessage.includes(keyword))
  );

  if (match) {
    return NextResponse.json({
      reply: match.reply,
      link: match.link
    });
  }

  return NextResponse.json({
    reply: "I'm your OHC Help Agent! I can help you with setting up your store, accepting payments, managing your AI team, or growing your business with marketing. What would you like to know more about?",
    link: {
      url: "/help",
      title: "Explore the full Help Center →"
    }
  });
}
