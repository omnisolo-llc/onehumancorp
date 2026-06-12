import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  // Simulate processing time
  await new Promise(resolve => setTimeout(resolve, 1500));

  return NextResponse.json({
    success: true,
    transcription: "Create a $150 repair quote",
    action: {
      type: 'quote_draft',
      details: {
        amount: 150,
        description: "Repair quote"
      }
    }
  });
}
