import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { command } = await req.json();

    if (!command) {
      return NextResponse.json({ error: 'Command is required' }, { status: 400 });
    }

    const lowerCmd = command.toLowerCase();

    if (lowerCmd.includes('sold out')) {
      return NextResponse.json({
        action: 'update_inventory',
        status: 'sold_out',
        target: 'chocolate cakes',
        message: 'Marked chocolate cakes as sold out via voice command.'
      });
    }

    return NextResponse.json({
      action: 'unknown',
      message: 'Processed voice command successfully.'
    });

  } catch (error) {
    return NextResponse.json({ error: 'Invalid request' }, { status: 400 });
  }
}
