import { NextResponse } from 'next/server';
import { Pool } from 'pg';
import crypto from 'crypto';

const pool = new Pool({
  connectionString: process.env.OHC_DATABASE_URL || 'postgres://postgres:postgres@localhost:5432/mono',
});

export async function POST(req: Request) {
  try {
    const body = await req.json();
    const { description = "", fileName, timestamp } = body;

    let isBookingRequest = false;
    let intentDraft = "Thank you for your inquiry. We will get back to you shortly.";

    const geminiApiKey = process.env.GEMINI_API_KEY;
    if (geminiApiKey) {
      try {
        const prompt = \`Evaluate the following customer message and determine if it is a "booking request" (e.g. asking for an appointment, time slot, service booking).
Respond with a JSON object with two fields:
- "isBookingRequest": true or false
- "draftedResponse": if it is a booking request, write a warm drafted response to the customer offering them 3 available time slots (e.g., Friday morning at 9am, 10am, and 11am). Otherwise, write a generic polite response.

Customer message: "\${description}"\`;

        const response = await fetch(
          \`https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key=\${geminiApiKey}\`,
          {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
              contents: [{ parts: [{ text: prompt }] }]
            }),
          }
        );

        if (response.ok) {
          const data = await response.json();
          const text = data.candidates?.[0]?.content?.parts?.[0]?.text || "";

          try {
             const jsonMatch = text.match(/\\{.*\\}/s);
             if (jsonMatch) {
               const parsed = JSON.parse(jsonMatch[0]);
               isBookingRequest = !!parsed.isBookingRequest;
               if (parsed.draftedResponse) {
                 intentDraft = parsed.draftedResponse;
               }
             }
          } catch (e) {
             console.error("Failed to parse Gemini response", text);
          }
        }
      } catch (err) {
        console.error("Error calling Gemini API:", err);
      }
    } else {
      // Fallback simple keyword match if no API key is available (for testing)
      const lowerDesc = description.toLowerCase();
      if (lowerDesc.includes('book') || lowerDesc.includes('appointment') || lowerDesc.includes('time') || lowerDesc.includes('fix') || lowerDesc.includes('schedule')) {
        isBookingRequest = true;
        intentDraft = "Hi there! I can certainly help with that. Are you available this Friday at 9am, 10am, or 11am?";
      }
    }

    if (isBookingRequest) {
      const tenantId = req.headers.get("x-tenant-id") || "default_tenant";
      const id = crypto.randomUUID();
      const department = "operations";
      const dbStatus = "DRAFT";
      const actionRisk = "HIGH";
      const payload = {
        feature_type: "booking_inquiry",
        username: "Customer",
        customer_inquiry: description,
        drafted_response: intentDraft,
        suggested_slots: ["Friday 9:00 AM", "Friday 10:00 AM", "Friday 11:00 AM"]
      };

      await pool.query(
        "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        [
          id,
          tenantId,
          department,
          \`Booking inquiry received: \${description.substring(0, 50)}...\`,
          dbStatus,
          actionRisk,
          JSON.stringify(payload)
        ]
      );
    }

    return NextResponse.json({
      success: true,
      request_id: 'mock_req_' + Date.now(),
      status: 'pending_agent_review'
    });
  } catch (e: any) {
    console.error("Booking request error:", e);
    return NextResponse.json({ error: e.message }, { status: 500 });
  }
}
