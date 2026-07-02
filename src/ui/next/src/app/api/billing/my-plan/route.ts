import { NextResponse } from 'next/server';

export async function GET(request: Request) {
    const authHeader = request.headers.get('Authorization');

    // Fallback to local rust server port 18789 if running standalone
    const backendUrl = process.env.NEXT_PUBLIC_API_URL || 'http://127.0.0.1:18789';

    try {
        const response = await fetch(`${backendUrl}/api/billing/my-plan`, {
            headers: {
                ...(authHeader ? { Authorization: authHeader } : {})
            }
        });

        if (!response.ok) {
            console.error('Failed to fetch from backend', response.status);
            return NextResponse.json({ error: 'Failed to fetch from backend' }, { status: response.status });
        }

        const data = await response.json();
        return NextResponse.json(data);
    } catch (error) {
        console.error('Error proxying to backend', error);
        return NextResponse.json({ error: 'Error proxying to backend' }, { status: 500 });
    }
}
