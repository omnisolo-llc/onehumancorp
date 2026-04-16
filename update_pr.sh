curl -s -X GET -H "Authorization: token $MY_GITHUB_TOKEN" \
  -H "Accept: application/vnd.github.v3+json" \
  "https://api.github.com/repos/onehumancorp/mono/pulls?head=onehumancorp:jules-16163251645015474908-241e6c8e"
