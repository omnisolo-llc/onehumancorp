import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json({
    nodes: [
      { id: "brain-01", type: "Brain", status: "online", load: "12%" },
      { id: "nerve-01", type: "Nerve", status: "online", load: "5%" },
      { id: "memory-01", type: "Memory", status: "online", load: "8%" }
    ]
  });
}
