import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  try {
    const backendUrl = process.env.OHC_BACKEND_URL || 'http://127.0.0.1:18789';

    // Parse query params to pass them to backend
    const { searchParams } = new URL(request.url);
    const tenant = searchParams.get('tenant') || 'embed';
    const theme = searchParams.get('theme') || 'light';

    const backendRes = await fetch(`${backendUrl}/api/v1/growth/review-wall/embed?tenant=${encodeURIComponent(tenant)}&theme=${encodeURIComponent(theme)}`, {
      method: 'GET',
    });

    if (backendRes.ok) {
        const text = await backendRes.text();
        return new NextResponse(text, {
          status: 200,
          headers: {
            'Content-Type': 'text/html',
          }
        });
    } else {
        return new NextResponse('Failed to load review wall widget', { status: backendRes.status });
    }
  } catch (error) {
    if (process.env.NODE_ENV !== "test") console.error("Error fetching review wall embed:", error);
    return new NextResponse('Internal Server Error', { status: 500 });
  }
}
