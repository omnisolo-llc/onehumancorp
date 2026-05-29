import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { name, subject, body: emailBody, target_segment } = body;

    // Simulate sending email
    console.log(`Sending email campaign: ${name} to ${target_segment}`);
    console.log(`Subject: ${subject}`);
    console.log(`Body: ${emailBody}`);

    // Introduce a short artificial delay to simulate processing
    await new Promise((resolve) => setTimeout(resolve, 500));

    return NextResponse.json({ success: true, message: 'Campaign sent successfully.' });
  } catch (error) {
    console.error('Error sending campaign:', error);
    return NextResponse.json(
      { error: 'Failed to send campaign' },
      { status: 500 }
    );
  }
}
