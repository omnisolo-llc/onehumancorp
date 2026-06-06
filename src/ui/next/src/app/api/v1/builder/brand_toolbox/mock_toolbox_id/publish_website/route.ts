import { NextResponse, NextRequest } from 'next/server';

export async function POST(request: NextRequest) {
  try {
    const mockSite = {
        domain: "luna-loaf.ohc.store"
    };
    return NextResponse.json(mockSite);
  } catch (e) {
    return NextResponse.json({}, { status: 500 });
  }
}
