import { NextResponse } from 'next/server';

let config = {
  is_enabled: false,
  custom_instructions: '',
  allow_appointments: false,
  allow_links: false,
};

export async function GET() {
  return NextResponse.json(config);
}

export async function POST(request: Request) {
  const data = await request.json();
  config = { ...config, ...data };
  return NextResponse.json({ status: 'success' });
}
