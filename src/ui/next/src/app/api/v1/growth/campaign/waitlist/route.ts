import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { campaignName, reward } = await request.json();
    const tenant = 'Store';

    const draft = `🚀 Join the exclusive Waitlist for ${campaignName || 'our upcoming launch'}!\n\n` +
      `We're building something amazing and want YOU to be the first to know.\n\n` +
      `Sign up for the waitlist today to secure your spot.\n\n` +
      `✨ Want to skip the line? Invite friends using your unique referral link and you'll unlock: ${reward || 'Early access and a special discount'}!\n\n` +
      `Click here to join the waitlist: [Waitlist Link]\n\n` +
      `Thanks for your support,\n` +
      `The ${tenant} Team`;

    return NextResponse.json({ message: draft });
  } catch (error) {
    return NextResponse.json({ error: "Failed to generate waitlist draft" }, { status: 500 });
  }
}
