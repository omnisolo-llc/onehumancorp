"""Exercise exact production request/fence SQL and immutable DDL, not Rust logic."""
from pathlib import Path
import re, sqlite3, tempfile
ROOT=Path(__file__).resolve().parents[2]
source=(ROOT/'src/server/orchestration/departments/manual_inbox.rs').read_text()
strings=re.findall(r'"([^"\n]*)"',source)
def exact(prefix):
    found=[s for s in strings if s.startswith(prefix)]
    assert len(found)==1,(prefix,found)
    return found[0]
insert=exact('INSERT INTO manual_inbox_requests(')
advance=exact('INSERT INTO manual_inbox_intents(')
claim=exact("UPDATE manual_inbox_requests SET state='claimed'")
retire=exact("UPDATE manual_inbox_requests SET state='retired' WHERE tenant_id=$1 AND inbox_message_id")
read=exact('SELECT request_id,actor_id,inbox_message_id,dispatch_id,source,recipient,body,provider_binding,payload_hash,intent,state,expires_at FROM manual_inbox_requests WHERE tenant_id=$1 AND actor_id')
fence=exact('INSERT INTO department_message_dispatches(')
finish=exact('UPDATE department_message_dispatches SET state=$1')
def params(request='request-a',tenant='a',actor='owner',inbox='inbox-a',revision=1):
    return (tenant,request,actor,inbox,'manual-'+request,'whatsapp','14155550123','Exact manual reply','{}','hash','send',revision,'pending',1,9999999999)
class NamedConnection(sqlite3.Connection):
    def execute(self,sql,parameters=()):
        if re.search(r'\$\d+',sql) and isinstance(parameters,tuple):
            parameters={str(i+1):v for i,v in enumerate(parameters)}
        return super().execute(sql,parameters)
count=0
def check(name,fn):
    global count
    fn();count+=1;print('PASS '+name)
with tempfile.TemporaryDirectory(prefix='manual-inbox-sqlite-') as directory:
    path=str(Path(directory)/'fixture.db');db=sqlite3.connect(path,factory=NamedConnection)
    for ddl in ['department_message_delivery_sqlite.sql','manual_inbox_requests_sqlite.sql']:
        db.executescript((ROOT/'src/server/persistence'/ddl).read_text())
    assert db.execute(advance,('a','inbox-a')).fetchone()==(1,)
    db.execute(insert,params());db.commit()
    def identity():
        for column in ['tenant_id','actor_id','request_id','inbox_message_id','dispatch_id','recipient','source','body','provider_binding','payload_hash','intent','intent_revision']:
            try:db.execute(f"UPDATE manual_inbox_requests SET {column}='changed'")
            except sqlite3.IntegrityError:pass
            else:raise AssertionError(column)
    check('actual DDL rejects every frozen identity mutation',identity)
    def retirement():
        assert db.execute(retire,('a','inbox-a')).rowcount==1
        assert db.execute(claim,('a','request-a',2)).rowcount==0
        assert db.execute(advance,('a','inbox-a')).fetchone()==(2,)
        db.execute(insert,params('request-b',revision=2))
        assert db.execute(claim,('a','request-b',2)).rowcount==1
    check('superseded pending identity cannot claim and revision is monotonic',retirement)
    def authority():
        for tenant,actor in [('other','owner'),('a','other')]:assert db.execute(read,(tenant,actor,'inbox-a',None)).fetchall()==[]
        assert db.execute(read,('a','owner','inbox-a',None)).fetchone()[0]=='request-b'
        assert db.execute(read,('a','owner','inbox-a','request-a')).fetchone()[0]=='request-a'
    check('actual read SQL scopes tenant actor message and optional identity',authority)
    def durable_fence():
        assert db.execute(fence,('a','manual-request-b','inbox-a','omni','hash','{}','unknown','pending',1)).rowcount==1
        assert db.execute(fence,('a','department-id','inbox-a','omni','hash','{}','unknown','pending',1)).rowcount==0
        db.commit();db.close()
    check('manual and department insert share one persisted unknown fence',durable_fence)
    db=sqlite3.connect(path,factory=NamedConnection)
    def restarted():
        assert db.execute(fence,('a','another-tab','inbox-a','omni','hash','{}','unknown','pending',1)).rowcount==0
        for tenant,action,hash_ in [('b','manual-request-b','hash'),('a','wrong','hash'),('a','manual-request-b','wrong')]:assert db.execute(finish,('accepted','wamid.fixture','accepted',2,tenant,action,hash_)).rowcount==0
        assert db.execute(finish,('accepted','wamid.fixture','accepted',2,'a','manual-request-b','hash')).rowcount==1
        assert db.execute(finish,('rejected',None,'rejected',3,'a','manual-request-b','hash')).rowcount==0
    check('restart retains fence and receipt completion requires exact claim',restarted)
    def tenants():
        assert db.execute(advance,('b','inbox-a')).fetchone()==(1,)
        db.execute(insert,params(tenant='b'))
        assert db.execute(read,('b','owner','inbox-a',None)).fetchone()[0]=='request-a'
    check('identical request and inbox identities stay separated by tenant',tenants)
    def no_effect():
        assert db.execute(fence,('a','rejected','new-inbox','omni','hash','{}','rejected','rejected',1)).rowcount==1
        assert db.execute(fence,('a','recovery','new-inbox','omni','hash','{}','unknown','pending',1)).rowcount==1
    check('proven no-effect outcome permits a new explicit attempt',no_effect)
    def rollback():
        db.commit();before=db.execute('SELECT revision FROM manual_inbox_intents WHERE tenant_id=? AND inbox_message_id=?',('a','inbox-a')).fetchone()
        db.execute('BEGIN');db.execute(advance,('a','inbox-a'));db.execute(retire,('a','inbox-a'));db.rollback()
        assert db.execute('SELECT revision FROM manual_inbox_intents WHERE tenant_id=? AND inbox_message_id=?',('a','inbox-a')).fetchone()==before
    check('failed admission rolls revision and pending-owner changes back',rollback)
    db.close()
print(f'{count} exact SQLite checks passed; Rust, SQLx and HTTP remain separate gates.')
