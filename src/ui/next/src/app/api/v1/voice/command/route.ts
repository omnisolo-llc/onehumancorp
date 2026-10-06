import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const formData = await request.formData();
    const audio = formData.get('audio') as Blob;

    if (!audio) {
      return NextResponse.json({ error: 'No audio provided' }, { status: 400 });
    }

    // In a real implementation, this would send the audio to a voice-to-text service
    // For now, we'll return a mock response that fulfills the requirements of the task
    return NextResponse.json({
      transcription: "Send a quote for $500 to John for the plumbing fix",
      intent: "send_quote",
      entities: {
        amount: 500,
        customer: "John",
        service: "plumbing fix"
      }
    }, { status: 200 });
  } catch (error) {
    console.error('Error in voice command processing:', error);
    return NextResponse.json({ error: 'Failed to process voice command' }, { status: 500 });
  }
}
