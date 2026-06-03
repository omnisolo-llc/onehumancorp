import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const body = await req.json();
    return NextResponse.json({ success: true, message: 'Settings saved' });
  } catch (e: any) {
    return NextResponse.json({ error: e.message }, { status: 500 });
  }
}
export async function GET(req: Request) {
  return NextResponse.json({ success: true, settings: { autonomousQuoting: false, basePricingRules: '' } });
}
