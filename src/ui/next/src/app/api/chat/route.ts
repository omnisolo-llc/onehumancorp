import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const { message } = await req.json();
  const text = message.toLowerCase();

  let linkUrl = "/help";
  let replyMsg = "I am your AI Help Agent! I specialize in answering questions about OHC features and helping you grow your small business. Check out our Getting Started guide.";

  if (text.includes("payment") || text.includes("stripe") || text.includes("paid")) {
    linkUrl = "/help/payments";
    replyMsg = "It looks like you have a question about payments! Setting up payments is easy. You can connect your Stripe account securely to start receiving money directly into your bank account.";
  } else if (text.includes("store") || text.includes("shop") || text.includes("start")) {
    linkUrl = "/help/getting-started";
    replyMsg = "Ready to build your store? Our AI can generate a complete storefront for you just by describing your business. Head over to the Store Builder to begin.";
  } else if (text.includes("agent") || text.includes("ai helper") || text.includes("assistant")) {
    linkUrl = "/help/ai-agents";
    replyMsg = "AI Agents are your automated workforce! You can hire different agents to handle marketing, customer support, and sales. They work 24/7 so you don't have to.";
  } else if (text.includes("market") || text.includes("customer") || text.includes("email")) {
    linkUrl = "/help/marketing";
    replyMsg = "Growing your business is easier than ever. You can use our marketing tools to send emails and promotions to your customers automatically.";
  } else if (text.includes("bill") || text.includes("account") || text.includes("plan")) {
    linkUrl = "/help/account-billing";
    replyMsg = "You can manage your subscription plan, view your billing history, and invite team members to your account from the Account Settings page.";
  }

  return NextResponse.json({
    reply: replyMsg,
    link: { url: linkUrl, title: "Read the full article →" }
  });
}
