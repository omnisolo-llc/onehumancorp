import { NextResponse } from 'next/server';
import tooltips from '../../../../tooltips.json';

export async function GET() {
  return NextResponse.json(tooltips);
}
