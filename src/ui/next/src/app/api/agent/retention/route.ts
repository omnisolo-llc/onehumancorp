import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { action, intervention_id } = await req.json();

    if (action === 'approve') {
      // Logic to approve the drafted message and send via SMS/Email
      // In a real implementation this would call the backend service to update status to APPROVED/SENT
      return NextResponse.json({ success: true, message: 'Offer sent successfully' });
    } else if (action === 'reject') {
      return NextResponse.json({ success: true, message: 'Offer rejected' });
    }

    return NextResponse.json({ error: 'Invalid action' }, { status: 400 });
  } catch (error) {
    return NextResponse.json({ error: 'Internal server error' }, { status: 500 });
  }
}
