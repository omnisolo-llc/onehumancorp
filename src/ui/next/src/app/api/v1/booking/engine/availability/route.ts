import { proxyBackendRequest } from "@/lib/auth/backendTransport";

const decoder = new TextDecoder("utf-8", { fatal: true });
const SAFE_ID = /^[A-Za-z0-9._-]{1,128}$/;

export async function POST(request: Request) {
  const response = await proxyBackendRequest(request, "/api/v1/booking/available_slots/invalid", {
    backendMethod: "GET",
    suppressRequestBody: true,
    resolveBackendPath(body) {
      const payload = JSON.parse(decoder.decode(body));
      if (!SAFE_ID.test(payload?.product_id)) throw new Error("invalid product id");
      const travel = payload?.travel_time === true ? "true" : "false";
      return `/api/v1/booking/available_slots/${payload.product_id}?travel_time=${travel}`;
    },
  });
  if (!response.ok) return response;
  try {
    const payload = await response.json();
    return Response.json({ available_slots: payload.slots ?? [] });
  } catch {
    return Response.json({ error: "Backend returned an invalid response" }, { status: 502 });
  }
}
