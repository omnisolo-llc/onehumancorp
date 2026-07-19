import { proxyBackendRequest } from "@/lib/auth/backendTransport";

const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();

export async function POST(request: Request) {
  const response = await proxyBackendRequest(request, "/api/v1/booking/reserve", {
    requestContentType: "application/json",
    transformRequestBody(body) {
      const payload = JSON.parse(decoder.decode(body));
      return encoder.encode(JSON.stringify({
        customer_name: payload.customer_name,
        customer_email: payload.customer_email,
        service_id: payload.product_id,
        start_time: payload.start_time,
        end_time: payload.end_time,
      }));
    },
  });
  if (!response.ok) return response;
  try {
    const payload = await response.json();
    return Response.json({
      success: payload.success,
      booking_id: payload.booking_id,
      deposit_stripe_link: payload.checkout_url,
    });
  } catch {
    return Response.json({ error: "Backend returned an invalid response" }, { status: 502 });
  }
}
