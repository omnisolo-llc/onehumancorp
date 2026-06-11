import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  try {
    const backendUrl = process.env.API_URL || 'http://localhost:18789';
    const res = await fetch(`${backendUrl}/api/v1/invoices`, {
      headers: {
        'Content-Type': 'application/json',
      },
    });

    if (!res.ok) {
        return NextResponse.json({ error: 'Failed to fetch invoices' }, { status: res.status });
    }

    const data = await res.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error('Error proxying GET /api/v1/invoices:', error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}

export async function POST(request: Request) {
  try {
    const backendUrl = process.env.API_URL || 'http://localhost:18789';
    const body = await request.json();

    const res = await fetch(`${backendUrl}/api/v1/invoices`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(body),
    });

    if (!res.ok) {
        return NextResponse.json({ error: 'Failed to create invoice' }, { status: res.status });
    }

    const data = await res.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error('Error proxying POST /api/v1/invoices:', error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
