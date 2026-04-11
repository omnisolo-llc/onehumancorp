package hybridfsmcp_test

import (
    "testing"
    "github.com/onehumancorp/mono/srcs/server/tools/hybridfsmcp"
)

func TestLocalFSProvider(t *testing.T) {
    tmp := t.TempDir()
    p := hybridfsmcp.NewLocalFSProvider(tmp)

    err := p.WriteFile("test.txt", []byte("hello"))
    if err != nil { t.Fatal(err) }

    data, err := p.ReadFile("test.txt")
    if err != nil || string(data) != "hello" { t.Fatal("read failed") }

    ls, err := p.ListDir(".")
    if err != nil || len(ls) != 1 || ls[0] != "test.txt" { t.Fatal("list failed") }

    err = p.WriteFile("../out.txt", []byte("bad"))
    if err == nil { t.Fatal("should fail on path traversal") }
}
