import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { incentiveEnabled, shareMessage, tenantId } = body;

    // Here you would typically save to Postgres via Prisma/Drizzle or the Rust backend
    // Since this is a standalone Next.js prototype setup, we'll mock a successful DB save.
    console.log(`[Growth Service] Saved Review Settings for tenant ${tenantId}:`, {
      incentiveEnabled,
      shareMessage,
    });

    return NextResponse.json({
      success: true,
      message: 'Review configuration saved successfully.',
      data: { incentiveEnabled, shareMessage }
    });

  } catch (error) {
    console.error('[Growth Service] Error saving review settings:', error);
    return NextResponse.json({ success: false, error: 'Failed to save settings' }, { status: 500 });
  }
}

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenantId = searchParams.get('tenantId') || 'my-store';

  // Mock fetching from DB
  return NextResponse.json({
    success: true,
    data: {
      incentiveEnabled: true,
      shareMessage: `I just had a fantastic experience with this store on OHC! Check them out: https://ohc.store/review-share?ref=${tenantId}`
    }
  });
}
