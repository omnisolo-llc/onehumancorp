import './globals.css';
import { FloatingHelpButton } from '@/components/help/FloatingHelpButton';
import { Walkthrough } from '@/components/help/Walkthrough';
import { Tooltip } from '@/components/help/Tooltip';

export const metadata = {
  title: 'OneHumanCorp App',
  description: 'Small business management platform',
};

const onboardingSteps = [
  { targetId: 'nav-dashboard', title: 'Welcome to your Dashboard', content: 'This is where you can see a high-level overview of your business performance.' },
  { targetId: 'nav-payments', title: 'Collect Payments', content: 'Send secure payment links to your customers with just an email address.' },
  { targetId: 'help-button-demo', title: 'Get Help Anytime', content: 'Click this button whenever you get stuck. Our AI agent and Help Center are available 24/7.' }
];

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <head>
        <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=Outfit:wght@400;600;700&display=swap" rel="stylesheet" />
      </head>
      <body className="bg-slate-50 text-slate-900 font-sans">
        <nav className="bg-white border-b border-slate-200 px-6 py-3 flex items-center justify-between sticky top-0 z-30">
          <div className="flex items-center gap-6">
            <h1 className="text-xl font-bold text-blue-600 tracking-tight" style={{ fontFamily: 'Outfit, sans-serif' }}>OneHumanCorp</h1>
            <div className="hidden md:flex items-center gap-4 text-sm font-medium text-slate-600">
              <span id="nav-dashboard" className="hover:text-blue-600 cursor-pointer p-2 rounded hover:bg-slate-50 transition-colors">Dashboard</span>
              <Tooltip id="tt-payments" text="Accept credit cards, bank transfers, and digital wallets instantly." position="bottom">
                <span id="nav-payments" className="hover:text-blue-600 cursor-pointer p-2 rounded hover:bg-slate-50 transition-colors">Payments</span>
              </Tooltip>
              <Tooltip id="tt-marketing" text="Send engaging email campaigns to your customers to increase sales." position="bottom">
                <span className="hover:text-blue-600 cursor-pointer p-2 rounded hover:bg-slate-50 transition-colors">Marketing</span>
              </Tooltip>
              <Tooltip id="tt-ai" text="Activate digital employees to handle customer support for you." position="bottom">
                <span className="hover:text-blue-600 cursor-pointer p-2 rounded hover:bg-slate-50 transition-colors">AI Agents</span>
              </Tooltip>
            </div>
          </div>
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 bg-indigo-100 text-indigo-700 rounded-full flex items-center justify-center font-bold text-sm">JS</div>
          </div>
        </nav>

        <main className="min-h-[calc(100vh-60px)]">
          {children}
        </main>

        <FloatingHelpButton />
        <Walkthrough flowId="initial-tour" steps={onboardingSteps} />

        {/* Dummy div to attach the last walkthrough step */}
        <div id="help-button-demo" className="fixed bottom-4 right-4 w-12 h-12 pointer-events-none opacity-0" />
      </body>
    </html>
  );
}
