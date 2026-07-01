import { NextResponse } from 'next/server';

export async function POST(req: Request, { params }: { params: { id: string } }) {
  try {
    const backendUrl = process.env.NEXT_PUBLIC_OHC_API_URL || 'http://localhost:18789';
    const id = params.id;

    const res = await fetch(`${backendUrl}/api/proposals/${id}/approve`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
    });

    if (!res.ok) {
      return NextResponse.json({ error: 'Failed to approve proposal' }, { status: res.status });
    }

    const data = await res.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error('Error approving proposal:', error);
    return NextResponse.json({ error: 'Internal server error' }, { status: 500 });
  }
}
