import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    const data = await request.json();

    // Simulate API delay
    await new Promise(resolve => setTimeout(resolve, 500));

    return NextResponse.json({ success: true, product: data });
}
