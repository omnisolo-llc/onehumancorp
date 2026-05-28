
'use client';
import { useRouter } from 'next/navigation';
import { useState } from 'react';

export default function Login() {
  const router = useRouter();
  const [showPassword, setShowPassword] = useState(false);

  const handleLogin = () => {
    // Navigate directly to dashboard to mock successful login
    router.push('/dashboard');
  };

  const handleStartSetup = () => {
    router.push('/business-setup');
  }

  return (
    <div>
      <h1>Login</h1>
      <div>One Human Corp</div>
      <input placeholder='Email or Username' />
      <input placeholder='Password' type={showPassword ? 'text' : 'password'} />
      <button onClick={handleLogin}>Login</button>
      <button>Sign Up</button>
      <div id='login-error'>Oops! We couldn't sign you in.</div>
      <button onClick={() => setShowPassword(!showPassword)}>Show</button>
      <button onClick={handleStartSetup}>Start Business Setup</button>
      <div>Create an account to start your business</div>
      <button>Have an account? Sign In</button>
    </div>
  );
}
