import { NextResponse } from 'next/server';

export const dynamic = 'force-dynamic';

export async function POST(request: Request) {
  try {
    const payload = await request.json();
    const platform = payload.platform;

    // Growth loop backend analytics tracking goes here.
    console.log(`[Growth] Post-purchase share recorded for platform: ${platform}`);

    return NextResponse.json({
      success: true,
      tracked: platform,
      reward: 10
    });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to process request' }, { status: 500 });
  }
}
