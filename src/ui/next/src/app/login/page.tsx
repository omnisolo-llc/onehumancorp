'use client';
import { useRouter } from 'next/navigation';

import { useState } from 'react';

export default function Login() {
  const router = useRouter();
  const [showSetup, setShowSetup] = useState(false);

  return (
    <div>
      {showSetup ? (
        <div>
          <h2>Your business, live in minutes.</h2>
          {/* Include other setup-related content or redirect here */}
        </div>
      ) : (
        <>
          <h1>Login</h1>
          <input type="text" placeholder="Email or Username" />
          <input type="password" placeholder="Password" />
          <button onClick={() => router.push('/dashboard')}>Log In</button>
          <br /><br />
          <button onClick={() => setShowSetup(true)}>Start Business Setup</button>
        </>
      )}
    </div>
  );
}
