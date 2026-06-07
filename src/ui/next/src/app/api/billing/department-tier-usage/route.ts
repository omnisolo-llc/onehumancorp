import { NextResponse } from 'next/server';

export async function GET(request: Request) {
    const authHeader = request.headers.get('Authorization');
    const backendUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

    try {
        const response = await fetch(`${backendUrl}/api/billing/department-tier-usage`, {
            headers: {
                ...(authHeader ? { Authorization: authHeader } : {})
            }
        });

        if (!response.ok) {
            console.error('Failed to fetch department tier usage from backend', response.status);
            return NextResponse.json({ error: 'Failed to fetch from backend' }, { status: response.status });
        }

        const data = await response.json();
        return NextResponse.json(data);
    } catch (error) {
        console.error('Error proxying department tier usage to backend', error);
        return NextResponse.json({ error: 'Error proxying to backend' }, { status: 500 });
    }
}
