import { NextResponse } from 'next/server';

export async function GET() {
  // Mock data representing what the Rust Chat Engine would return via gRPC/REST gateway.
  // In a fully integrated environment, we'd use a generated gRPC web client here.
  return NextResponse.json({
    conversations: [
      {
        id: "mock-conv-1",
        contact_id: "mock-contact-1",
        contact_name: "Maya",
        status: "OPEN",
        latest_message: {
          content: "Do you make vegan cakes?",
          sender_type: "CONTACT"
        },
        ai_draft: "Hi Maya! Yes, we absolutely make vegan cakes. We have a delicious vegan chocolate and a vegan vanilla bean option. Would you like to place a custom order?"
      }
    ]
  });
}
