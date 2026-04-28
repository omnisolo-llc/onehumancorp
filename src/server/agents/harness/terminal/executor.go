package terminal

type Executor struct {
	Validator CommandValidator
}

func NewExecutor() *Executor {
	return &Executor{
		Validator: NewTokenValidator(),
	}
}

func (e *Executor) Execute(command string) error {
	if err := e.Validator.Validate(command); err != nil {
		return err
	}
	return nil
}
