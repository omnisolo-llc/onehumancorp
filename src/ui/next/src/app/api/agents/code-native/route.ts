import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    // Mocking the backend implementation of CodeNativePipeline for the UI
    const results = [
        "Generated rich data with ID: test_id",
        "Processed data natively. New record count: 2"
    ];

    return NextResponse.json({ results });
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}
