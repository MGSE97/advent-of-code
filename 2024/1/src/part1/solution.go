package part1

import (
	"fmt"

	"github.com/spf13/cobra"
)

func resolve(cmd *cobra.Command, args []string) {
	fmt.Println("1 called")
}