import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { token, action } = await request.json();

    if (!token || !action) {
      return NextResponse.json({ error: 'Token and action are required' }, { status: 400 });
    }

    if (!['approve', 'reject'].includes(action)) {
      return NextResponse.json({ error: 'Invalid action' }, { status: 400 });
    }

    // In a full implementation, this would send the approval to the Rust backend
    // For now, we simulate the backend processing.

    await new Promise(resolve => setTimeout(resolve, 800));

    const actionText = action === 'approve' ? 'Approved and sent quote' : 'Rejected quote draft';

    return NextResponse.json({
      success: true,
      message: actionText
    });

  } catch (e: any) {
    console.error(`Action API error: ${e.message}`);
    return NextResponse.json({ error: 'Failed to process action request' }, { status: 500 });
  }
}
