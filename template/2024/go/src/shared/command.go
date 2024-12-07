package shared

import (
	"fmt"

	"github.com/spf13/cobra"
)

func CreateResolver(part int, question string, answer string, resolver func(cmd *cobra.Command, args []string) string) cobra.Command {
	handler := func(cmd *cobra.Command, args []string) {
		fmt.Printf("Part %d.:\n", part)
		fmt.Println("----------------------------------------------------------------------------------------------------------------")
		solution := resolver(cmd, args)
		fmt.Println("----------------------------------------------------------------------------------------------------------------")
		fmt.Printf("Part %d.\nQ: %s\nA: %s\n", part, question, fmt.Sprintf(answer, solution))
	}

	return cobra.Command{
		Use:   string(part+int('0')),
		Short: question,
		Long: `Gets answer for following question, using provided configuration file.
		"{{question}}"`,
		Run: handler,
	}
}