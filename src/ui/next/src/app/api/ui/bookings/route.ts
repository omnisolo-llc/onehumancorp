import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenantId = searchParams.get('tenant_id');

  if (!tenantId) {
    return NextResponse.json({ error: 'tenant_id is required' }, { status: 400 });
  }

  // Mock data for bookings to avoid depending on an actual DB setup for this CUJ right away
  const mockBookings = [
    {
      id: 'apt-1',
      customer_name: 'Sarah Connor',
      product_title: 'Guitar Lesson',
      start_time: new Date(Date.now() + 1000 * 60 * 60 * 2).toISOString(), // 2 hours from now
      status: 'confirmed',
      payment_status: 'unpaid',
      notes: 'She struggled with chords last week.',
      ai_summary: '3rd lesson. Focus: Jazz scales.',
    },
    {
      id: 'apt-2',
      customer_name: 'John Smith',
      product_title: 'Plumbing Repair',
      start_time: new Date(Date.now() + 1000 * 60 * 60 * 24).toISOString(), // Tomorrow
      status: 'pending',
      payment_status: 'deposit_required',
      notes: 'Leak in the kitchen sink.',
      ai_summary: 'First time customer. Might need pipe replacement.',
    }
  ];

  return NextResponse.json(mockBookings);
}
