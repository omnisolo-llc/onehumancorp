#!/bin/bash
cat << 'JSON_EOF' > package.json.tmp
{
  "pnpm": {
    "overrides": {
      "ws": "^6.2.4",
      "fast-uri": "^3.1.4",
      "axios": "^1.18.0",
      "immutable": "^4.3.9",
      "next": "^16.2.11",
      "@hono/node-server": "^2.0.5",
      "dompurify": "^3.4.12",
      "sharp": "^0.35.3",
      "brace-expansion": "^5.0.7"
    }
  }
}
JSON_EOF
jq 'del(.overrides)' package.json > package.json.tmp2
mv package.json.tmp2 package.json
jq -s '.[0] * .[1]' package.json package.json.tmp > package.json.new
mv package.json.new package.json
rm package.json.tmp

pnpm update
npm audit
