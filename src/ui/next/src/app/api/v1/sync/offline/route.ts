import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const data = await request.json();
    console.log("Received offline sync batch:", data);
    return NextResponse.json({ success: true, processed: data.transactions?.length || 0 });
  } catch (error) {
    return NextResponse.json({ success: false, error: "Failed to process batch" }, { status: 500 });
  }
}
