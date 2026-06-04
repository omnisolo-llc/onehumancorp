/** @type {import('next').NextConfig} */
const nextConfig = {
  async rewrites() {
    return [
      {
        source: '/api/v1/growth/:path*',
        destination: process.env.BACKEND_URL ? `${process.env.BACKEND_URL}/api/v1/growth/:path*` : 'http://localhost:8080/api/v1/growth/:path*',
      },
    ]
  },
}

export default nextConfig;
