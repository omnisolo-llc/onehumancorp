"use client";
import { useRouter } from 'next/navigation';

export default function Home() {
  const router = useRouter();

  return (
    <div>
      <nav>Nav</nav>
      <h1>One Human Corp</h1>
      <button onClick={() => router.push('/onboarding')}>🚀 Start Business Setup</button>
      <button onClick={() => router.push('/onboarding')}>🚀 Start My Business</button>
      <button onClick={() => router.push('/onboarding')}>Start Setup</button>

      <button onClick={() => router.push('/onboarding')}>⚡ Instant Build (AI) →</button>
      <button>Fix App Issues</button>
      <button>Continue with Google/Apple</button>
      <p>Your business, live in minutes.</p>
    </div>
  );
}
