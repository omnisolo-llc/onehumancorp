import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const body = await req.json();
    const tenantId = req.headers.get('X-Tenant-ID') || 'storefront';
    const userId = req.headers.get('X-User-ID') || 'test-user';
    const backendUrl = process.env.OHC_CORE_URL || 'http://127.0.0.1:18789';

    console.log(`[API] Migrating platform from URL: ${body.url}`);

    const res = await fetch(`${backendUrl}/api/onboarding/migrate`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'X-Tenant-ID': tenantId,
        'X-User-ID': userId,
      },
      body: JSON.stringify(body)
    });

    if (!res.ok) {
      console.error(`[API] Backend migrate failed with status ${res.status}`);
      return NextResponse.json({ error: 'Backend migration failed' }, { status: res.status });
    }

    const data = await res.json();
    return NextResponse.json(data);
  } catch (error: any) {
    console.error('[API] Migrate error:', error);
    return NextResponse.json(
      { error: 'Internal server error during migration' },
      { status: 500 }
    );
  }
}
