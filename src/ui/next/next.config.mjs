/** @type {import('next').NextConfig} */
const nextConfig = {
  allowedDevOrigins: ['http://127.0.0.1:18789', '127.0.0.1', 'localhost'],
  outputFileTracingRoot: new URL('../../../', import.meta.url).pathname,
}

export default nextConfig;
