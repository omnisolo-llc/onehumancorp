/** @type {import('next').NextConfig} */
const nextConfig = {
  allowedDevOrigins: ['http://127.0.0.1:18789', '127.0.0.1', 'localhost'],
  async rewrites() {
    return [
      {
        source: '/api/settings/:path*',
        destination: process.env.BACKEND_URL ? `${process.env.BACKEND_URL}/api/settings/:path*` : 'http://localhost:8080/api/settings/:path*',
      },
      {
        source: '/api/ui/:path*',
        destination: process.env.BACKEND_URL ? `${process.env.BACKEND_URL}/api/ui/:path*` : 'http://localhost:8080/api/ui/:path*',
      },
      {
        source: '/api/v1/growth/:path*',
        destination: process.env.BACKEND_URL ? `${process.env.BACKEND_URL}/api/v1/growth/:path*` : 'http://localhost:8080/api/v1/growth/:path*',
      },
      {
        source: '/api/v1/builder/:path*',
        destination: process.env.BACKEND_URL ? `${process.env.BACKEND_URL}/api/v1/builder/:path*` : 'http://localhost:8080/api/v1/builder/:path*',
      },
      {
        source: '/api/v1/dashboard/:path*',
        destination: process.env.BACKEND_URL ? `${process.env.BACKEND_URL}/api/v1/dashboard/:path*` : 'http://localhost:8080/api/v1/dashboard/:path*',
      },
      {
        source: '/api/v1/onboarding/:path*',
        destination: process.env.BACKEND_URL ? `${process.env.BACKEND_URL}/api/v1/onboarding/:path*` : 'http://localhost:8080/api/v1/onboarding/:path*',
      },
      {
        source: '/api/billing/:path*',
        destination: process.env.BACKEND_URL ? `${process.env.BACKEND_URL}/api/billing/:path*` : 'http://localhost:8080/api/billing/:path*',
      },
      {
        source: '/api/agent-feed/:path*',
        destination: process.env.BACKEND_URL ? `${process.env.BACKEND_URL}/api/agent-feed/:path*` : 'http://localhost:8080/api/agent-feed/:path*',
      },
      {
        source: '/api/dev/:path*',
        destination: process.env.BACKEND_URL ? `${process.env.BACKEND_URL}/api/dev/:path*` : 'http://localhost:8080/api/dev/:path*',
      }
    ]
  },
}

export default nextConfig;
