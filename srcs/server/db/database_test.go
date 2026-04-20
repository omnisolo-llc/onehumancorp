package db

import (
	"reflect"
	"testing"
)

func TestSplitSQLStatements(t *testing.T) {
	input := `
CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);
INSERT INTO users(name) VALUES ('A;B');
-- keep this together;
INSERT INTO users(name) VALUES ('C');
/* multi; line; comment */
UPDATE users SET name = "semi;colon" WHERE id = 1;
`

	got := splitSQLStatements(input)
	want := []string{
		"CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)",
		"INSERT INTO users(name) VALUES ('A;B')",
		"-- keep this together;\nINSERT INTO users(name) VALUES ('C')",
		"/* multi; line; comment */\nUPDATE users SET name = \"semi;colon\" WHERE id = 1",
	}

	if !reflect.DeepEqual(got, want) {
		t.Fatalf("splitSQLStatements() = %#v, want %#v", got, want)
	}
}

func TestAppendSQLiteKeyPragmaQuotesKey(t *testing.T) {
	dsn := appendSQLiteKeyPragma("/tmp/ohc_state.db?_pragma=busy_timeout(15000)", "0730757854de7cd4")
	want := "/tmp/ohc_state.db?_pragma=busy_timeout(15000)&_pragma=key('0730757854de7cd4')"
	if dsn != want {
		t.Fatalf("appendSQLiteKeyPragma() = %q, want %q", dsn, want)
	}
}
