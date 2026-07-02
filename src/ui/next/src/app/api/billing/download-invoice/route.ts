import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const backendUrl = process.env.BACKEND_URL || process.env.NEXT_PUBLIC_API_URL || 'http://127.0.0.1:18789';

    const headers: Record<string, string> = {};

    const authHeader = req.headers.get('Authorization');
    if (authHeader) {
      headers['Authorization'] = authHeader;
    }

    const res = await fetch(`${backendUrl}/api/billing/download-invoice`, {
      method: 'POST',
      headers,
    });

    if (!res.ok) {
        console.error('Failed to download invoice on backend', res.status);
        return NextResponse.json({ error: 'Failed to download invoice' }, { status: res.status });
    }

    const data = await res.json();
    return NextResponse.json(data, { status: res.status });
  } catch (error) {
    if (process.env.NODE_ENV !== "test") console.warn('Warn proxying to backend:', error);
    return NextResponse.json(
      { message: 'Internal Server Error' },
      { status: 500 }
    );
  }
}
