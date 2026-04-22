package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	filePath := "srcs/server/db/postgres_provider.go"
	contentBytes, err := ioutil.ReadFile(filePath)
	if err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		return
	}
	content := string(contentBytes)

    // replace NewPgProvider
    if !strings.Contains(content, "func NewPgProvider(pool *pgxpool.Pool, redisClient rueidis.Client)") {
        content = strings.Replace(content, "func NewPgProvider(pool *pgxpool.Pool) *PgProvider {", "func NewPgProvider(pool *pgxpool.Pool, redisClient rueidis.Client) *PgProvider {", 1)
        content = strings.Replace(content, "return &PgProvider{pool: pool}", "return &PgProvider{pool: pool, redisClient: redisClient}", 1)
        err = ioutil.WriteFile(filePath, []byte(content), 0644)
		if err != nil {
			fmt.Printf("Error writing file: %v\n", err)
			return
		}
		fmt.Println("Successfully modified NewPgProvider")
    }

	newClaimTask := `
func (p *PgProvider) ClaimTask(ctx context.Context, taskID string) error {
	if p.redisClient != nil {
		lockKey := "task_lock:" + taskID
		cmd := p.redisClient.B().Set().Key(lockKey).Value("locked").Nx().ExSeconds(30).Build()
		err := p.redisClient.Do(ctx, cmd).Error()
		if err != nil {
			if rueidis.IsRedisNil(err) {
				return fmt.Errorf("task %s is locked by another agent", taskID)
			}
			return err
		}
		defer func() {
			delCmd := p.redisClient.B().Del().Key(lockKey).Build()
			_ = p.redisClient.Do(ctx, delCmd).Error()
		}()
	}

	query := ` + "`" + `
		UPDATE tasks
		SET status = 'IN_PROGRESS', updated_at = CURRENT_TIMESTAMP
		WHERE id = (
			SELECT id FROM tasks
			WHERE id = $1 AND status = 'PENDING'
			FOR UPDATE SKIP LOCKED
		)
	` + "`" + `
	tag, err := p.pool.Exec(ctx, query, taskID)
	if err != nil {
		return err
	}
	if tag.RowsAffected() == 0 {
		return fmt.Errorf("task %s not found or already claimed", taskID)
	}
	return nil
}
`
    // replace ClaimTask
    if strings.Contains(content, "func (p *PgProvider) ClaimTask(ctx context.Context, taskID string) error {") {
        idx := strings.Index(content, "func (p *PgProvider) ClaimTask(ctx context.Context, taskID string) error {")
        endIdx := strings.Index(content[idx:], "\n}\n") + idx + 3
        content = content[:idx] + newClaimTask + content[endIdx:]
        err = ioutil.WriteFile(filePath, []byte(content), 0644)
		if err != nil {
			fmt.Printf("Error writing file: %v\n", err)
			return
		}
		fmt.Println("Successfully modified ClaimTask")
    }
}
