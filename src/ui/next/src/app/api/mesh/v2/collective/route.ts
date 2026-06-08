import { NextResponse } from 'next/server';
import { cookies } from 'next/headers';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const action = searchParams.get('action');

  const tenant = "test_tenant";

  if (action === 'getNearby') {
    // Ideally this would call the Rust gRPC backend
    // Since we mock it in Rust, we'll just mock the response here
    return NextResponse.json({
      success: true,
      neighbors: ["carlos_repairs", "fatima_food_cart"]
    });
  }

  return NextResponse.json({ success: false, error: 'Unknown action' }, { status: 400 });
}

export async function POST(request: Request) {
  const body = await request.json();
  const action = body.action;

  if (action === 'invite') {
    // Calling the Rust gRPC backend directly from Next.js is hard without a proper gRPC client
    // For now we'll mock the success response
    return NextResponse.json({
      success: true
    });
  }

  return NextResponse.json({ success: false, error: 'Unknown action' }, { status: 400 });
}
