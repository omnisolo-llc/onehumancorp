#include "srcs/server/agents/builtin/http_client.h"

#include <chrono>
#include <cstdint>
#include <string>

#include "absl/status/status.h"
#include "absl/strings/ascii.h"
#include "absl/strings/match.h"
#include "absl/strings/str_cat.h"
#include "httplib.h"

namespace ohc::agent {
namespace {

struct ParsedUrl {
  std::string base_url;
  std::string path;
};

absl::StatusOr<ParsedUrl> ParseUrl(const std::string& url) {
  absl::string_view remaining(url);
  absl::string_view scheme;
  if (absl::StartsWith(remaining, "http://")) {
    scheme = "http://";
  } else if (absl::StartsWith(remaining, "https://")) {
    scheme = "https://";
  } else {
    return absl::InvalidArgumentError(
        absl::StrCat("unsupported URL scheme: ", url));
  }
  remaining.remove_prefix(scheme.size());

  if (remaining.empty()) {
    return absl::InvalidArgumentError(
        absl::StrCat("URL is missing an authority: ", url));
  }

  const size_t authority_end = remaining.find_first_of("/?#");
  const absl::string_view authority = remaining.substr(0, authority_end);
  if (authority.empty()) {
    return absl::InvalidArgumentError(
        absl::StrCat("URL is missing an authority: ", url));
  }

  absl::string_view path = "/";
  if (authority_end != absl::string_view::npos) {
    path = remaining.substr(authority_end);
  }

  const size_t fragment = path.find('#');
  if (fragment != absl::string_view::npos) {
    path = path.substr(0, fragment);
  }

  std::string normalized_path(path);
  if (normalized_path.empty()) {
    normalized_path = "/";
  } else if (normalized_path.front() != '/') {
    normalized_path.insert(normalized_path.begin(), '/');
  }

  return ParsedUrl{
      .base_url = absl::StrCat(scheme, authority),
      .path = std::move(normalized_path),
  };
}

httplib::Headers MakeDefaultHeaders(
    const std::map<std::string, std::string>& headers,
    std::string* content_type) {
  httplib::Headers result;
  for (const auto& [key, value] : headers) {
    if (absl::EqualsIgnoreCase(key, "content-type")) {
      *content_type = value;
      continue;
    }
    result.emplace(key, value);
  }
  return result;
}

absl::StatusOr<httplib::Result> DispatchRequest(httplib::Client* client,
                                                const HttpRequest& req,
                                                const ParsedUrl& parsed_url) {
  std::string content_type;
  client->set_default_headers(MakeDefaultHeaders(req.headers, &content_type));

  const std::string method = absl::AsciiStrToUpper(req.method);
  if (method == "GET") {
    return client->Get(parsed_url.path);
  }
  if (method == "POST") {
    if (content_type.empty()) {
      content_type = "application/octet-stream";
    }
    return client->Post(parsed_url.path, req.body, content_type);
  }
  if (method == "PUT") {
    if (content_type.empty()) {
      content_type = "application/octet-stream";
    }
    return client->Put(parsed_url.path, req.body, content_type);
  }
  if (method == "PATCH") {
    if (content_type.empty()) {
      content_type = "application/octet-stream";
    }
    return client->Patch(parsed_url.path, req.body, content_type);
  }
  if (method == "DELETE") {
    return client->Delete(parsed_url.path);
  }
  if (method == "OPTIONS") {
    return client->Options(parsed_url.path);
  }

  return absl::InvalidArgumentError(
      absl::StrCat("unsupported HTTP method: ", req.method));
}

absl::Status MakeTransportError(const httplib::Result& result) {
  std::string message =
      absl::StrCat("HTTP request failed: ", httplib::to_string(result.error()));
#ifdef CPPHTTPLIB_OPENSSL_SUPPORT
  if (result.error() == httplib::Error::SSLConnection ||
      result.error() == httplib::Error::SSLLoadingCerts ||
      result.error() == httplib::Error::SSLServerVerification ||
      result.error() == httplib::Error::SSLServerHostnameVerification) {
    absl::StrAppend(&message, " (tls_backend_error=", result.ssl_backend_error(),
                    ")");
  }
#endif
  return absl::InternalError(message);
}

}  // namespace

absl::StatusOr<HttpResponse> HttpDo(const HttpRequest& req) {
  auto parsed_url = ParseUrl(req.url);
  if (!parsed_url.ok()) {
    return parsed_url.status();
  }

  httplib::Client client(parsed_url->base_url);
  if (!client.is_valid()) {
    return absl::InvalidArgumentError(
        absl::StrCat("invalid URL: ", req.url));
  }

  const auto timeout = std::chrono::seconds(req.timeout_seconds);
  client.set_follow_location(true);
  client.set_keep_alive(false);
  client.set_path_encode(false);
  client.set_connection_timeout(timeout);
  client.set_read_timeout(timeout);
  client.set_write_timeout(timeout);
  client.set_max_timeout(timeout);

  auto response = DispatchRequest(&client, req, *parsed_url);
  if (!response.ok()) {
    return response.status();
  }
  if (!*response) {
    return MakeTransportError(*response);
  }

  return HttpResponse{
      .status_code = static_cast<int32_t>((*response)->status),
      .body = std::move((*response)->body),
  };
}

}  // namespace ohc::agent
