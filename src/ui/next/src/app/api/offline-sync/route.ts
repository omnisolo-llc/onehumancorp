import { NextResponse } from 'next/server';
import { cookies } from 'next/headers';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const cookieStore = cookies();
    const ohcSession = cookieStore.get('ohc_session')?.value;

    const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

    const response = await fetch(`${apiUrl}/api/offline-sync`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...(ohcSession ? { Cookie: `ohc_session=${ohcSession}` } : {}),
      },
      body: JSON.stringify(body),
    });

    if (!response.ok) {
       const errorData = await response.text();
       console.error('Backend offline sync failed:', response.status, errorData);
       return NextResponse.json({ error: 'Backend sync failed' }, { status: response.status });
    }

    const data = await response.json();
    return NextResponse.json(data);

  } catch (error) {
    console.error('Error in offline sync proxy:', error);
    return NextResponse.json({ error: 'Internal server error' }, { status: 500 });
  }
}
