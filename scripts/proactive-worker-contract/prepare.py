"""Compile the unchanged actual spawned worker with its real database/cache/auth types."""
from pathlib import Path
import hashlib,json
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
def item(path, marker, attributes=False):
    source=(ROOT/path).read_text()
    assert source.count(marker)==1, marker
    start=source.index(marker)
    if attributes:
        start=source.rfind('#[derive',0,start)
        assert start>=0
    brace=source.index('{',start);depth=1;end=brace+1
    while depth:
        if source[end]=='{':depth+=1
        elif source[end]=='}':depth-=1
        end+=1
    return source[start:end]
parts=['pub mod db { use sqlx::{PgPool,SqlitePool};', item('src/server/db.rs','pub enum DbStore',True),item('src/server/db.rs','pub struct DB {',True),'}']
cache=(ROOT/'src/server/utils/cache.rs').read_text().split('#[cfg(test)]')[0]
parts+=['pub mod cache {',cache,'}']
parts+=['pub mod api { pub mod agent_feed { use serde::{Serialize,Deserialize}; use chrono::{DateTime,Utc}; use sqlx::FromRow; use std::sync::{Arc,OnceLock}; use crate::cache::HybridCache;']
for path,marker in [('src/server/domain/repository/agent_feed_repo.rs','pub struct AgentFeedItem {'),('src/server/api/agent_feed.rs','pub struct MobileAgentFeedItem {'),('src/server/api/agent_feed.rs','pub struct MobileAgentFeedListResponse {'),('src/server/api/agent_feed.rs','pub enum AnyAgentFeedListResponse {'),('src/server/api/agent_feed.rs','pub struct AgentFeedListResponse {')]:
    parts.append(item(path,marker,True))
api=(ROOT/'src/server/api/agent_feed.rs').read_text()
parts.append(next(line for line in api.splitlines() if line.startswith('pub static AGENT_FEED_CACHE:')))
parts+=[item('src/server/api/agent_feed.rs','pub fn get_agent_feed_cache()'),' } }']
parts+=['pub mod workers {']
for name in ['proactive_operations_storage','proactive_operations_polling','proactive_operations_worker']:
    parts.append(f'#[path={json.dumps(str(ROOT / "src/server/workers" / (name+".rs")))}] pub mod {name};')
parts+=['}', '#[cfg(test)] #[path="test.rs"] mod tests;',
    '#[cfg(test)] use workers::{proactive_operations_storage as storage, proactive_operations_polling as polling};',
    '#[cfg(test)] #[path="postgres.rs"] mod postgres;',
    '#[cfg(test)] #[path="fairness.rs"] mod fairness;',
    '''#[cfg(test)] fn test_database_url() -> String {
        use std::str::FromStr;
        let url=std::env::var("OHC_OPS_PROBE_DB").expect("owned loopback PostgreSQL required");
        let options=sqlx::postgres::PgConnectOptions::from_str(&url).expect("valid PostgreSQL URL");
        assert!(["127.0.0.1","localhost"].contains(&options.get_host()));
        assert_eq!(options.get_database(),Some("ohc_proactive_probe"));
        url
    }''']
(HERE/'generated.rs').write_text('\n'.join(parts))
inputs=[ROOT/'Cargo.toml',ROOT/'scripts/focused_ci_gate.py',ROOT/'scripts/test_focused_ci_gate.py',ROOT/'.github/workflows/ci.yml',ROOT/'src/server/common/Cargo.toml',ROOT/'src/server/config/Cargo.toml',ROOT/'Cargo.lock',ROOT/'src/server/db.rs',ROOT/'src/server/api/agent_feed.rs',ROOT/'src/server/domain/repository/agent_feed_repo.rs',ROOT/'src/server/utils/cache.rs']
inputs += list((ROOT/'src/server/common').rglob('*.rs'))+list((ROOT/'src/server/config').rglob('*.rs'))
inputs += list((ROOT/'src/server/workers').glob('proactive_operations*.rs'))
inputs += [p for p in HERE.iterdir() if p.is_file() and p.name not in {'generated.rs','source-manifest.json'}]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))},indent=2)+'\n')
