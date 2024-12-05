package part2

import (
	"github.com/spf13/cobra"
)

var Cmd = &cobra.Command{
	Use:   "2",
	Short: "Get answer for part 2.",
	Long:  `Gets answer for part 2, using provided configuration file.`,
	Run:   resolve,
}

func init() {
}
