import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';

// Execute the production portable SQL, not a copy of its statements. This is a
// SQLite SQL/storage check; the Rust/PostgreSQL transaction tests are separate.
test('actual inventory CAS and receipt SQL preserve tenant ownership, zero and atomic rollback in SQLite', () => {
  const result = spawnSync('python3', ['-c', String.raw`
import pathlib,re,sqlite3,tempfile,json
source=pathlib.Path('src/server/api/pos_inventory.rs').read_text()
def sql(name):
 m=re.search(r'const '+name+r': &str = "([^"]+)"',source)
 assert m, 'missing production SQL '+name
 return m.group(1)
with tempfile.TemporaryDirectory() as directory:
 path=directory+'/inventory.db'
 db=sqlite3.connect(path)
 db.executescript('CREATE TABLE products(id TEXT PRIMARY KEY,tenant_id TEXT,inventory_count INTEGER,available_quantity INTEGER,is_sold_out BOOLEAN,updated_at TEXT);CREATE TABLE inventory_levels(id TEXT,tenant_id TEXT,available_count INTEGER,updated_at TEXT);')
 migration=pathlib.Path('src/server/migrations/1041_inventory_adjustment_receipts.sql').read_text()
 db.executescript(migration.split('ALTER TABLE')[0])
 db.execute("INSERT INTO products VALUES('owned','a',3,3,1,'v1')")
 db.execute("INSERT INTO inventory_levels VALUES('level','a',3,'v1')")
 db.commit()
 def update(id='owned',tenant='a',old=3,new=4):
  return db.execute(sql('UPDATE_PRODUCT'),{'1':new,'2':new,'3':None,'4':'v2','5':id,'6':tenant,'7':old}).rowcount
 assert update('missing')==0
 assert update(tenant='b')==0
 assert update(old=2)==0
 assert db.execute('SELECT count(*) FROM products').fetchone()[0]==1
 db.rollback()
 assert update(new=0)==1
 db.execute(sql('INSERT_RECEIPT'),{'1':'a','2':'mutation','3':'owned','4':'request','5':'{"stock":0}'})
 db.commit();db.close()
 db=sqlite3.connect(path)
 assert db.execute('SELECT inventory_count,is_sold_out FROM products').fetchone()==(0,1)
 assert db.execute('SELECT receipt_json FROM inventory_adjustment_receipts').fetchone()[0]=='{"stock":0}'
 try:
  assert update(old=0,new=1)==1
  db.execute(sql('INSERT_RECEIPT'),{'1':'a','2':'mutation','3':'owned','4':'changed','5':'{}'})
  db.commit()
  raise AssertionError('duplicate identity must fail')
 except sqlite3.IntegrityError: db.rollback()
 assert db.execute('SELECT inventory_count FROM products').fetchone()[0]==0
 assert db.execute(sql('UPDATE_LEVEL'),{'1':-4,'2':'v2','3':'level','4':'a'}).rowcount==0
 assert db.execute(sql('UPDATE_LEVEL'),{'1':-1,'2':'v2','3':'level','4':'b'}).rowcount==0
 assert db.execute(sql('UPDATE_LEVEL'),{'1':-3,'2':'v2','3':'level','4':'a'}).rowcount==1
 db.commit()
 assert db.execute('SELECT available_count FROM inventory_levels').fetchone()[0]==0
 print('SQLite ownership, CAS, zero, persisted receipt, duplicate rollback and stock floor passed')
`], { cwd: new URL('..', import.meta.url), encoding: 'utf8' });
  assert.equal(result.status, 0, result.stdout + result.stderr);
});
