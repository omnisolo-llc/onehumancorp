package main
import "os"
import "strings"
func main() {
	content, _ := os.ReadFile("srcs/server/orchestration/queue_v2_test.go")
	newContent := strings.Replace(string(content), "db.NewTestProvider(t)", "db.NewSqliteProvider(db.CreateTestSqliteDB(t))", 1)
	os.WriteFile("srcs/server/orchestration/queue_v2_test.go", []byte(newContent), 0644)
}
