import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const backendUrl = process.env.OHC_BACKEND_URL || 'http://127.0.0.1:18789';
    const body = await request.json();

    const headers = new Headers({
      'Content-Type': 'application/json',
    });
    const authHeader = request.headers.get('authorization');
    if (authHeader) {
      headers.set('authorization', authHeader);
    }
    const cookie = request.headers.get('cookie');
    if (cookie) {
      headers.set('cookie', cookie);
    }

    const backendRes = await fetch(`${backendUrl}/api/v1/growth/referrals/click`, {
      method: 'POST',
      headers,
      body: JSON.stringify(body)
    });

    if (backendRes.ok) {
        return NextResponse.json({ success: true });
    } else {
        return NextResponse.json(
            { error: 'Failed to record referral click' },
            { status: backendRes.status }
        );
    }
  } catch (error) {
    if (process.env.NODE_ENV !== "test") console.error("Error recording referral click:", error);
    return NextResponse.json(
        { error: 'Internal Server Error' },
        { status: 500 }
    );
  }
}

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const target = searchParams.get('target') || '/onboarding';
  const ref = searchParams.get('ref');

  try {
    const backendUrl = process.env.OHC_BACKEND_URL || 'http://localhost:8080';

    if (ref) {
        // Record the click on the backend if we have a ref
        const headers = new Headers({
          'Content-Type': 'application/json',
        });
        const authHeader = request.headers.get('authorization');
        if (authHeader) headers.set('authorization', authHeader);
        const cookie = request.headers.get('cookie');
        if (cookie) headers.set('cookie', cookie);

        // Best effort async fetch
        fetch(`${backendUrl}/api/v1/growth/referrals/click`, {
          method: 'POST',
          headers,
          body: JSON.stringify({ id: ref })
        }).catch(err => console.error("Error recording referral click on GET:", err));
    }

    // Redirect to the target
    const redirectUrl = new URL(target, request.url);
    if (ref) redirectUrl.searchParams.set('ref', ref);

    return NextResponse.redirect(redirectUrl);
  } catch (error) {
    console.error("Error redirecting referral click:", error);
    return NextResponse.redirect(new URL('/onboarding', request.url));
  }
}
