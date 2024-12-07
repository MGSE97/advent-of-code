package part1

import (
	"lets/src/shared"

	"github.com/spf13/cobra"
)

func Resolve(cmd *cobra.Command, args []string) string {
	// Todo: Put resolver code here
	input := shared.ReadFile("input")
	return input
}