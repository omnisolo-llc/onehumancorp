import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { messages } = await req.json();

    // Simulate AI drafting a reply based on the context of the most recent message
    const aiDraft = "Yes, we have several vegan birthday cake options available! You can order them directly from our website or let me know what flavors you are interested in.";

    return NextResponse.json({
      draft: aiDraft
    });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to generate draft' }, { status: 500 });
  }
}
