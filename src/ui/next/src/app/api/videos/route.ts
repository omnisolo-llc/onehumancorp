import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  try {
    const mockVideos = [
      { id: 1, title: 'How to add a product', duration: '1:20' },
      { id: 2, title: 'Setting up payments', duration: '1:15' },
    ];
    return NextResponse.json(mockVideos);
  } catch (e) {
    console.error("Failed to fetch videos from backend:", e);
    return NextResponse.json([], { status: 500 });
  }
}
