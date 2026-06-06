/** @type {import('next').NextConfig} */
const nextConfig = {
  async rewrites() {
    return [
      {
        source: '/api/ui/:path*',
        destination: process.env.BACKEND_URL ? `${process.env.BACKEND_URL}/api/ui/:path*` : 'http://localhost:8080/api/ui/:path*',
      },
      {
        source: '/api/v1/growth/:path*',
        destination: process.env.BACKEND_URL ? `${process.env.BACKEND_URL}/api/v1/growth/:path*` : 'http://localhost:8080/api/v1/growth/:path*',
      },
      {
        source: '/api/billing/:path*',
        destination: process.env.BACKEND_URL ? `${process.env.BACKEND_URL}/api/billing/:path*` : 'http://localhost:8080/api/billing/:path*',
      },
      {
        source: '/api/v1/shipping/:path*',
        destination: process.env.BACKEND_URL ? `${process.env.BACKEND_URL}/api/v1/shipping/:path*` : 'http://localhost:8080/api/v1/shipping/:path*',
      },
      {
        source: '/api/v1/booking/:path*',
        destination: process.env.BACKEND_URL ? `${process.env.BACKEND_URL}/api/v1/booking/:path*` : 'http://localhost:8080/api/v1/booking/:path*',
      },
      {
        source: '/api/subscriptions/:path*',
        destination: process.env.BACKEND_URL ? `${process.env.BACKEND_URL}/api/subscriptions/:path*` : 'http://localhost:8080/api/subscriptions/:path*',
      }
    ]
  },
}

export default nextConfig;
