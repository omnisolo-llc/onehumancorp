import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json({
    tasks: [
      { id: "1", title: "Inventory Reorder Strategy", status: "queued", priority: "P1" }
    ]
  });
}
