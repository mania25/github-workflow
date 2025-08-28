/** @type {import('next').NextConfig} */
const nextConfig = {
  experimental: {
    appDir: true,
  },
  output: 'standalone',
  env: {
    BACKEND_URL: process.env.BACKEND_URL || 'http://localhost:8080',
  },
}

module.exports = nextConfig