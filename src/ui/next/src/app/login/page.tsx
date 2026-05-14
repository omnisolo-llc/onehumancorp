import React from 'react';

export default function Login() {
  return (
    <div style={{ padding: '20px', minHeight: '100vh', background: 'linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%)' }}>
      <h1>Login</h1>
      <form action="/">
        <input type="text" placeholder="Email or Username" style={{ display: 'block', margin: '10px 0', padding: '10px' }} />
        <input type="password" placeholder="Password" style={{ display: 'block', margin: '10px 0', padding: '10px' }} />
        <button type="submit" style={{ padding: '10px 20px' }}>Sign In</button>
      </form>
    </div>
  );
}
