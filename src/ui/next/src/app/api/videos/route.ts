import { NextResponse, NextRequest } from 'next/server';
import { fetchApi } from '../../../lib/fetchApi';

export async function GET(request: NextRequest) {
  try {
    const res = await fetchApi('/api/videos');

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json([], { status: res.status });
  } catch (e) {
    console.error("Failed to fetch videos from backend:", e);
    return NextResponse.json([], { status: 500 });
  }
}
