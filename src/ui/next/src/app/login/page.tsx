'use client';
import { useRouter } from 'next/navigation';
import { useState } from 'react';

import { useState } from 'react';

export default function Login() {
  const router = useRouter();
<<<<<<< HEAD
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
=======
  const [email, setEmail] = useState('');

  const handleLogin = () => {
    if (email) {
      document.cookie = `user_email=${encodeURIComponent(email)}; path=/; max-age=86400`;
      document.cookie = `tenant_id=tenant-${encodeURIComponent(email)}; path=/; max-age=86400`;
    }
    router.push('/dashboard');
  };

  return (
    <div>
      <h1>Login</h1>
      <input
        type="text"
        placeholder="Email or Username"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
      />
      <input type="password" placeholder="Password" />
      <button onClick={handleLogin}>Login</button>
>>>>>>> 8819bf4c (fix: ensure cross-device persistence for onboarding state)
    </div>
  );
}
