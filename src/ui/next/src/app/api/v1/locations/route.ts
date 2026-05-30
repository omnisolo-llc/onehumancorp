import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const data = await req.json();
    console.log("Mock provisioning new location:", data);
    return NextResponse.json({ success: true, node_id: "mock_node_id_123", name: data.name });
  } catch (e) {
    return NextResponse.json({ error: "Failed to provision" }, { status: 500 });
  }
}
