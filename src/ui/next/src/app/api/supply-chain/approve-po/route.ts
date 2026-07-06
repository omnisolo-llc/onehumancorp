import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { poId } = await req.json();

    // Ideally this would call the Rust backend via fetch()
    // For now we simulate the approval
    console.log(`Approving PO ${poId} in supply chain mesh`);

    return NextResponse.json({ success: true, poId });
  } catch (err: any) {
    return NextResponse.json({ error: err.message }, { status: 500 });
  }
}
