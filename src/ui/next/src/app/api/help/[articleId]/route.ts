import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest, context: { params: Promise<{ articleId: string }> }) {
    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    const p = await context.params;

    try {
        const res = await fetch(`${backendUrl}/api/help/${p.articleId}`);
        if (!res.ok) {
             return NextResponse.json({ error: 'Article not found' }, { status: 404 });
        }
        const data = await res.json();
        return NextResponse.json(data);
    } catch (error) {
        return NextResponse.json({ error: 'Article not found' }, { status: 404 });
    }
}
