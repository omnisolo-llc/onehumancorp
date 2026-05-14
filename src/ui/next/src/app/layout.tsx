import './globals.css';
import { HelpChatWidget } from '../components/HelpChatWidget';

export const metadata = {
  title: 'One Human Corp',
  description: 'Small Business App',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        {children}
        <HelpChatWidget />
      </body>
    </html>
  );
}
