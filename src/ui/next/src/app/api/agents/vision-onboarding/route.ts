import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  // Simulating processing of an image upload to extract menu items
  return NextResponse.json({
    extracted_text: 'Chicken Shawarma Plate $12\nVegan Wrap $9'
  });
}
