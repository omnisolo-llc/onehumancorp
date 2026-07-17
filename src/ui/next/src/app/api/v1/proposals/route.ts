import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import {
  invalidProposalId,
  validateLegacyProposalBody,
  proposalBackendPath,
  proposalIdFromUrl,
} from "./proposalBackend";

function pathFromRequest(request: Request): string | Response {
  try {
    const id = proposalIdFromUrl(request.url);
    return id === null ? "/api/v1/proposals" : proposalBackendPath(id);
  } catch {
    return invalidProposalId();
  }
}

export function GET(request: Request): Promise<Response> | Response {
  const path = pathFromRequest(request);
  return typeof path === "string"
    ? proxyBackendRequest(request, path, {
        forwardQuery: false,
        requestContentType: "application/json",
      })
    : path;
}

export function POST(request: Request): Promise<Response> | Response {
  const path = pathFromRequest(request);
  if (typeof path !== "string") return path;
  const updating = path !== "/api/v1/proposals";
  return proxyBackendRequest(request, path, {
    backendMethod: updating ? "PUT" : "POST",
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: validateLegacyProposalBody,
  });
}
