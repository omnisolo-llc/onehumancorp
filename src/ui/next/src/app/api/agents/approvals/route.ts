import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  // Always return mock data for local dev and tests so we can see the UI
  const mockApprovals = {
    pending_approvals: [
      {
        id: "1",
        tenant_id: "default",
        department: "operations",
        description: "Review drafted message to user | Payload: {\"original_message\":\"Hi, where is my cake?\",\"generated_response\":\"Your cake is on the way!\"}",
        status: "Pending",
        action_risk: "low",
        payload: {
          feature_type: "ambassador_reply",
          original_message: "Hi, where is my cake?",
          generated_response: "Your cake is on the way!"
        }
      }
    ]
  };
  return NextResponse.json(mockApprovals);
}
