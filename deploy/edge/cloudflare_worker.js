// Cloudflare Worker script for OHC Storefronts
// Resolves custom domains to OHC tenant IDs and serves edge-cached HTML.
export default {
  async fetch(request, env, ctx) {
    const url = new URL(request.url);
    const host = url.hostname;

    // 1. Lookup tenant_id by custom domain in Edge KV
    let tenantId = await env.OHC_DOMAINS.get(host);

    // 2. Fallback to API resolution if not in KV
    if (!tenantId) {
      const resolveUrl = new URL(`/api/v1/storefront/resolve?domain=${host}`, "https://api.onehumancorp.com");
      const resolveRes = await fetch(resolveUrl);
      if (resolveRes.ok) {
        const data = await resolveRes.json();
        tenantId = data.tenant_id;
        // Cache the resolution asynchronously
        ctx.waitUntil(env.OHC_DOMAINS.put(host, tenantId, { expirationTtl: 3600 }));
      } else {
        return new Response("Domain not mapped", { status: 404 });
      }
    }

    // 3. Rewrite the URL to hit the OHC backend rendering service
    const backendUrl = new URL(url.pathname, "https://api.onehumancorp.com");
    if (url.pathname.startsWith('/product/')) {
       const productId = url.pathname.split('/')[2];
       backendUrl.pathname = `/api/v1/storefront/${tenantId}/${productId}`;
    } else {
       // Assume root or home page rendering if at root, else preserve path
       if (url.pathname === '/' || url.pathname === '') {
         backendUrl.pathname = `/api/v1/storefront/${tenantId}/home`;
       } else {
         backendUrl.pathname = `/api/v1/storefront/${tenantId}${url.pathname}`;
       }
    }

    const modifiedRequest = new Request(backendUrl.toString(), request);

    // 4. Fetch from backend, which returns Cache-Control, Surrogate-Key, ETag etc.
    let response = await fetch(modifiedRequest);

    return response;
  }
}
