const BASE_DEV_ORIGINS = ['127.0.0.1', 'localhost']

function isPrivateLanHostname(hostname) {
  const octets = hostname.split('.').map(Number)
  if (
    octets.length === 4 &&
    octets.every((octet) => Number.isInteger(octet) && octet >= 0 && octet <= 255)
  ) {
    return octets[0] === 10 ||
      (octets[0] === 172 && octets[1] >= 16 && octets[1] <= 31) ||
      (octets[0] === 192 && octets[1] === 168)
  }

  const bareIpv6 = hostname.startsWith('[') && hostname.endsWith(']')
    ? hostname.slice(1, -1).toLowerCase()
    : hostname.toLowerCase()
  if (!bareIpv6.includes(':')) return false
  const firstGroup = Number.parseInt(bareIpv6.split(':', 1)[0], 16)
  return Number.isFinite(firstGroup) &&
    ((firstGroup & 0xfe00) === 0xfc00 || (firstGroup & 0xffc0) === 0xfe80)
}

export function allowedDevOrigins(environment = process.env) {
  if (environment.OHC_WEB_LOCAL_DEV !== 'true') return [...BASE_DEV_ORIGINS]

  try {
    const canonical = new URL(environment.OHC_WEB_CANONICAL_ORIGIN ?? '')
    if (canonical.protocol !== 'http:' || !isPrivateLanHostname(canonical.hostname)) {
      return [...BASE_DEV_ORIGINS]
    }
    return [...BASE_DEV_ORIGINS, canonical.hostname]
  } catch {
    return [...BASE_DEV_ORIGINS]
  }
}

/** @type {import('next').NextConfig} */
const nextConfig = {
  allowedDevOrigins: allowedDevOrigins(),
  outputFileTracingRoot: new URL('../../../', import.meta.url).pathname,
<<<<<<< HEAD
  typescript: {
    ignoreBuildErrors: true,
  },
  eslint: {
    ignoreDuringBuilds: true,
  },
=======
>>>>>>> 97cc191c1 (perf: tokio RwLock, Redis pool, SSE streaming, unified WS, backpressure, React hooks)
}

export default nextConfig;
