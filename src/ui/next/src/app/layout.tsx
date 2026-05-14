import { HelpCenterWidget } from '../components/help/HelpCenterWidget';

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>
        {children}
        <HelpCenterWidget />
      </body>
    </html>
  )
}
