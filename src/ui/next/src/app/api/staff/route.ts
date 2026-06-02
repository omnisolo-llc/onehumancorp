import { NextResponse } from 'next/server';

let mockStaffDB = [
  { id: 'staff-1', name: 'Sarah', role: 'Cashier', pin: '1234' }
];

export async function GET() {
  return NextResponse.json({ staff: mockStaffDB });
}

export async function POST(request: Request) {
  const body = await request.json();

  if (body.action === 'verify_pin') {
    const user = mockStaffDB.find(s => s.pin === body.pin);
    if (user) {
      return NextResponse.json({ success: true, user });
    }
    return NextResponse.json({ success: false }, { status: 401 });
  }

  if (body.action === 'create') {
    const newUser = {
      id: crypto.randomUUID(),
      name: body.name || 'New Staff',
      role: body.role,
      pin: body.pin || '0000'
    };
    mockStaffDB.push(newUser);
    return NextResponse.json({ success: true, user: newUser });
  }

  return NextResponse.json({ error: 'Invalid action' }, { status: 400 });
}
