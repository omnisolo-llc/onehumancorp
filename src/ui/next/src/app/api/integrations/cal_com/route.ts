import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const body = await req.json();

    // Simulate Cal.com API call to connect calendar
    await new Promise(resolve => setTimeout(resolve, 500));

    return NextResponse.json({
      success: true,
      message: 'Connected to Cal.com successfully',
    });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to connect Cal.com' }, { status: 500 });
  }
}
