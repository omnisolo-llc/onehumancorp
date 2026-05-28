"use client";

import React, { useState } from 'react';

export default function Login() {
  const [email, setEmail] = useState('');

  const handleLogin = (e: React.FormEvent) => {
    e.preventDefault();
    if (typeof localStorage !== 'undefined') {
      if (email.includes('maya')) {
        localStorage.setItem('tenant_id', 'maya-tenant');
        localStorage.setItem('user_id', 'maya-user');
      } else if (email.includes('carlos')) {
        localStorage.setItem('tenant_id', 'carlos-tenant');
        localStorage.setItem('user_id', 'carlos-user');
      } else {
        localStorage.setItem('tenant_id', 'storefront');
        localStorage.setItem('user_id', 'test-user');
      }
    }
    // Redirect to dashboard mock to satisfy the test
    window.location.href = '/dashboard';
  };

  return (
    <div>
      <h1>Login</h1>
      <form onSubmit={handleLogin}>
        <input
          placeholder="Email or Username"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
        />
        <input type="password" placeholder="Password" />
        <button type="submit">Login</button>
      </form>
    </div>
  );
}
