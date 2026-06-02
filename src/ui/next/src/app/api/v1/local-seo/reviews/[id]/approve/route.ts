import { NextResponse } from 'next/server';

export async function POST(
  request: Request,
  { params }: { params: { id: string } }
) {
    try {
        const body = await request.json();
        return NextResponse.json({
            status: 'success',
            message: 'Reply published successfully',
            review_id: params.id,
            published_reply: body.reply
        });
    } catch (e) {
        return NextResponse.json({
            status: 'error',
            message: 'Failed to publish reply'
        }, { status: 400 });
    }
}
