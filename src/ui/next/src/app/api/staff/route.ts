import { NextResponse } from 'next/server';

let staffMembers: any[] = [];

export async function GET() {
  return NextResponse.json(staffMembers);
}

export async function POST(request: Request) {
  const body = await request.json();
  const newStaff = {
    id: `staff_${Date.now()}`,
    ...body,
    pin_hash: '1234', // In a real app, securely generate and hash PINs
    created_at: new Date().toISOString()
  };
  staffMembers.push(newStaff);
  return NextResponse.json(newStaff, { status: 201 });
}
