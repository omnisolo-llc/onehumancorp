#pragma once

// Thin libcurl wrapper used by the LLM client implementations.
// Using libcurl keeps the binary small and is pre-installed on virtually
// every Linux / Raspberry-Pi distribution.

#include <cstdint>
#include <map>
#include <string>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"

namespace ohc::agent {

struct HttpRequest {
  std::string                        url;
  std::string                        method = "GET";  // "GET" or "POST"
  std::string                        body;
  std::map<std::string, std::string> headers;
  int32_t                            timeout_seconds = 120;
};

struct HttpResponse {
  int32_t     status_code = 0;
  std::string body;
};

// Performs an HTTP request synchronously.
// Returns an error Status on network failure; HTTP-level errors are surfaced
// via HttpResponse::status_code so the caller can decide how to handle them.
absl::StatusOr<HttpResponse> HttpDo(const HttpRequest& req);

}  // namespace ohc::agent
