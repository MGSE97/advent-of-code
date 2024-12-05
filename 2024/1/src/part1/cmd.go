package part1

import (
	"github.com/spf13/cobra"
)

var Cmd = &cobra.Command{
	Use:   "1",
	Short: "Get answer for part 1.",
	Long:  `Gets answer for part 1, using provided configuration file.`,
	Run:   resolve,
}

func init() {
}
