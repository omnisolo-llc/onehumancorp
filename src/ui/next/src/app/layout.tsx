import './globals.css'
import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import { OfflineSyncIndicator } from '@/components/OfflineSyncIndicator'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'One Human Corp',
  description: 'AI Work Assistant for Owners',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        {children}
        <OfflineSyncIndicator />
      </body>
    </html>
  )
}
