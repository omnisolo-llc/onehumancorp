#include "srcs/server/agents/builtin/http_client.h"

#include <curl/curl.h>

#include "absl/status/status.h"
#include "absl/strings/str_cat.h"

namespace ohc::agent {
namespace {

// libcurl write-callback: appends received bytes to a std::string.
// libcurl guarantees size==1 but we use size*nmemb for correctness.
size_t CurlWriteCallback(char* ptr, size_t size, size_t nmemb,
                         std::string* output) {
  const size_t total = size * nmemb;
  output->append(ptr, total);
  return total;
}

}  // namespace

absl::StatusOr<HttpResponse> HttpDo(const HttpRequest& req) {
  CURL* curl = curl_easy_init();
  if (!curl) {
    return absl::InternalError("curl_easy_init failed");
  }
  // RAII guard: always call curl_easy_cleanup.
  struct CurlGuard {
    CURL* c;
    ~CurlGuard() { curl_easy_cleanup(c); }
  } guard{curl};

  HttpResponse response;

  curl_easy_setopt(curl, CURLOPT_URL, req.url.c_str());
  curl_easy_setopt(curl, CURLOPT_TIMEOUT, static_cast<long>(req.timeout_seconds));
  curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, CurlWriteCallback);
  curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response.body);
  curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1L);
  // On embedded systems the CA bundle path can differ; let libcurl use its
  // compiled-in default.  Override via CURL_CA_BUNDLE env if needed.
  curl_easy_setopt(curl, CURLOPT_SSL_VERIFYPEER, 1L);

  // Build header list.
  struct curl_slist* headers = nullptr;
  // RAII guard for curl_slist.
  struct HeaderGuard {
    struct curl_slist* h = nullptr;
    ~HeaderGuard() {
      if (h) curl_slist_free_all(h);
    }
  } hguard;

  for (const auto& [key, value] : req.headers) {
    std::string header = absl::StrCat(key, ": ", value);
    headers = curl_slist_append(headers, header.c_str());
  }
  hguard.h = headers;
  if (headers) {
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
  }

  if (req.method == "POST") {
    curl_easy_setopt(curl, CURLOPT_POST, 1L);
    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, req.body.c_str());
    curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE,
                     static_cast<long>(req.body.size()));
  }

  CURLcode res = curl_easy_perform(curl);
  if (res != CURLE_OK) {
    return absl::InternalError(
        absl::StrCat("libcurl error: ", curl_easy_strerror(res)));
  }

  long http_code = 0;
  curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
  response.status_code = static_cast<int32_t>(http_code);

  return response;
}

}  // namespace ohc::agent
