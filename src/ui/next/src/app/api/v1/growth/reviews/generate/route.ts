import { NextResponse } from 'next/server';

const reviewRequests = [
  "Hi {name}! 🌟 We hope you're loving your recent purchase from {tenant}. Could you take 60 seconds to leave us a quick review? It helps our small business grow! Reply to this email or click here: https://ohc.store/review/{tenant}",
  "Hey {name}! Thank you so much for choosing {tenant}. Your support means the world to us! 💖 If you have a moment, we'd love to hear your feedback. Drop a review here: https://ohc.store/review/{tenant}",
  "Hi {name}, checking in to see how you're enjoying your order from {tenant}! 🛍️ If you're happy with it, a quick review would make our day. Leave a review: https://ohc.store/review/{tenant}",
  "Hello {name}! As a small business, word of mouth is our biggest growth driver. 🚀 If you loved your {tenant} experience, please share it with others! Review link: https://ohc.store/review/{tenant}",
  "Hi {name}! 🎁 A huge thank you for your order! We strive for 5-star experiences. If we hit the mark, please let us know with a review: https://ohc.store/review/{tenant}"
];

export async function POST(request: Request) {
  try {
    const { tenant, customerName } = await request.json();
    const tenantName = tenant || 'our store';
    const name = customerName || 'there';

    // Pick a random review request
    const randomRequest = reviewRequests[Math.floor(Math.random() * reviewRequests.length)];
    const message = randomRequest.replace(/{tenant}/g, tenantName).replace(/{name}/g, name);

    // Simulate API delay
    await new Promise(resolve => setTimeout(resolve, 800));

    return NextResponse.json({ message });
  } catch (error) {
    console.error("Error generating review request:", error);
    return NextResponse.json(
      { error: "Failed to generate review request" },
      { status: 500 }
    );
  }
}
