package solve

import (
	"lets/src/part1"
	"lets/src/part2"

	"github.com/spf13/cobra"
)

// solveCmd represents the solve command
var Cmd = &cobra.Command{
	Use:   "solve",
	Short: "Solve answer for selected part.",
	Long:  "Runs code and gets answer for selected part.",
	PersistentPreRun: preRun,
}

var cfgFile string

func init() {
	rootCmd.AddCommand(Cmd)
	
	Cmd.AddCommand(part1.Cmd)
	Cmd.AddCommand(part2.Cmd)
	
	Cmd.PersistentFlags().StringVarP(&cfgFile, "config", "c", "", "config file (default is ./data/part_X.toml)")
}


func preRun(cmd *cobra.Command, args []string) {
	var part int
	if cmd == part1.Cmd { 
		part = 1 
	} else if cmd == part2.Cmd { 
		part = 2 
	}
	initConfig(cfgFile, part)
}