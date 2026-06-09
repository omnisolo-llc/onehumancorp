import { NextResponse, NextRequest } from 'next/server';
import { cookies } from 'next/headers';

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const q = searchParams.get('q') || '';

  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  const cookieStore = cookies();
  const token = cookieStore.get('auth_token')?.value;

  try {
    const res = await fetch(`${backendUrl}/api/v1/search?q=${encodeURIComponent(q)}`, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token || ''}`,
      },
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json([], { status: res.status });
  } catch (e) {
    if (process.env.NODE_ENV !== "test") console.error("Failed to fetch search from backend:", e);
    return NextResponse.json([], { status: 500 });
  }
}
