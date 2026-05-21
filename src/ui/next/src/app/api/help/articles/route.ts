import { NextResponse } from 'next/server';

export async function GET() {
  const articles = [
    {
      id: "getting-started-1",
      topic: "Getting Started",
      title: "Welcome to One Human Corp",
      content_markdown: "Welcome to One Human Corp! This is a simple app that helps you manage your small business. You can set up your store, accept payments, and hire AI helpers."
    },
    {
      id: "my-store-1",
      topic: "My Store",
      title: "Setting up your storefront",
      content_markdown: "To set up your storefront, go to the 'My Store' tab and add your products. It's easy! Just upload a photo, write a simple description, and set a price."
    },
    {
      id: "payments-1",
      topic: "Payments",
      title: "Accepting your first payment",
      content_markdown: "When a customer buys something, the money goes straight to your account. We handle all the technical details so you can focus on your business."
    },
    {
      id: "ai-agents-1",
      topic: "AI Agents",
      title: "Activating your AI Support Agent",
      content_markdown: "Need a hand? Your AI Support Agent can answer customer emails and chats for you while you sleep. Just turn it on in the 'AI Agents' tab."
    },
    {
      id: "marketing-1",
      topic: "Marketing",
      title: "Creating a social media post",
      content_markdown: "Let our AI write your social media posts! Just tell it what you want to sell, and it will give you a catchy post to share with your customers."
    },
    {
      id: "account-billing-1",
      topic: "Account & Billing",
      title: "Understanding your invoice",
      content_markdown: "Your monthly invoice shows exactly what you paid for. We keep things simple with no hidden fees."
    }
  ];

  return NextResponse.json(articles);
}
