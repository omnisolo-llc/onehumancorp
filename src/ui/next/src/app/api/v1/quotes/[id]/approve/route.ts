import { NextRequest, NextResponse } from 'next/server';

export async function PATCH(req: NextRequest, { params }: { params: { id: string } }) {
  const { id } = params;

  const apiUrl = process.env.OHC_API_URL || 'http://localhost:8080';

  try {
    const res = await fetch(`${apiUrl}/quotes/${id}/approve`, {
      method: 'PATCH',
      headers: {
        'Content-Type': 'application/json',
      },
    });

    if (!res.ok) {
      return NextResponse.json({ error: 'Failed to approve quote' }, { status: res.status });
    }

    const data = await res.json();
    return NextResponse.json(data);
  } catch (err) {
    console.error('Proxy to Rust backend failed', err);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
