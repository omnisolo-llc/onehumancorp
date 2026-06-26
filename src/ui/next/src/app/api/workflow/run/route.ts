import { NextRequest, NextResponse } from 'next/server';

export const runtime = 'nodejs';

export async function POST(request: NextRequest) {
  try {
    const payload = await request.json();

    const backendPayload = payload;

    const API_BASE = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:18789';
    const response = await fetch(`${API_BASE}/api/workflow/run`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(backendPayload),
    });

    if (!response.ok) {
      const text = await response.text();
      console.error('Visual workflow api err response from backend:', text);
      throw new Error(`Backend returned ${response.status}: ${text}`);
    }

    const data = await response.json();
    return NextResponse.json(data, { status: 200 });

  } catch (error: any) {
    console.error('Error running workflow:', error);
    return NextResponse.json({ success: false, error: error.message }, { status: 500 });
  }
}
