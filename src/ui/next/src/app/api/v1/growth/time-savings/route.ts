import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json({
    hours_saved: 12,
    inquiries_handled: 48,
    appointments_scheduled: 14,
    carts_recovered: 2,
    auto_replied: 40
  });
}
