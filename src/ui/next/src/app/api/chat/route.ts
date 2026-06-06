import { NextResponse } from 'next/server';

const MAX_MESSAGE_LENGTH = 1000;

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
  let reply = "I am your AI Help Agent! I specialize in answering questions about OHC features and helping you grow your small business. Check out our Getting Started guide.";
  let link = { url: "/help", title: "Read the full article →" };

  if (query.includes("subscription") || query.includes("recurring") || query.includes("monthly")) {
    reply = "Setting up subscriptions or recurring lesson packages is easy! Leo uses this to bill his students automatically every month. Check out Leo's Guide to get started.";
    link = { url: "/help/leo-guide", title: "Read Leo's Subscription Guide →" };
  } else if (query.includes("cake") || query.includes("custom") || query.includes("deposit")) {
    reply = "Maya uses our 'Custom Order' feature to take cake orders with upfront deposits. This ensures you're paid for your hard work! Learn how in Maya's Guide.";
    link = { url: "/help/maya-guide", title: "Read Maya's Custom Order Guide →" };
  } else if (query.includes("repair") || query.includes("handyman") || query.includes("booking")) {
    reply = "Carlos manages his handyman repairs using our Booking system. You can set your hours and let customers book times that work for you. See Carlos's Guide.";
    link = { url: "/help/carlos-guide", title: "Read Carlos's Booking Guide →" };
  } else if (query.includes("food") || query.includes("pickup") || query.includes("cart")) {
    reply = "Fatima runs her food cart using 'Pickup Only' mode. It helps her manage busy lunch rushes without a hitch. Check out Fatima's Guide for more.";
    link = { url: "/help/fatima-guide", title: "Read Fatima's Pickup Guide →" };
  }

  return NextResponse.json({ reply, link });
}
