'use client';
import { useRouter } from 'next/navigation';
import { useState } from 'react';

export default function Login() {
  const router = useRouter();
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
    </div>
  );
}
