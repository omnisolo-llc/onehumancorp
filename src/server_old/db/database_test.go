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
