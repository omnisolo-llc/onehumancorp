import Link from 'next/link';

export default function Home() {
  return (
    <main style={{ padding: '20px' }}>
      <h1>Home Page</h1>
      <nav>
        <ul>
          <li>
            <Link href="/dashboards/competitor-audit" id="nav-competitor-audit">
              Competitor Audit
            </Link>
          </li>
        </ul>
      </nav>
    </main>
  );
}
