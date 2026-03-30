import re

with open('srcs/orchestration/sip.go', 'r') as f:
    content = f.read()

content = content.replace(
    'db, err := sql.Open("sqlite", dbPath)',
    '''
	dsn := dbPath
	if !strings.Contains(dsn, "?") {
		dsn += "?_pragma=journal_mode(WAL)&_pragma=busy_timeout(15000)&_txlock=immediate"
	} else {
		dsn += "&_pragma=journal_mode(WAL)&_pragma=busy_timeout(15000)&_txlock=immediate"
	}
	db, err := sql.Open("sqlite", dsn)
	if err == nil {
		db.SetMaxOpenConns(1)
	}'''
)

with open('srcs/orchestration/sip.go', 'w') as f:
    f.write(content)
