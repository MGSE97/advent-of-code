package solve

import (
	"lets/src/part1"
	"lets/src/part2"
	"lets/src/shared"

	"github.com/spf13/cobra"
)

func createResolvers() map[int]cobra.Command {
	return map[int]cobra.Command{
		1: shared.CreateResolver(
			1, 
			"What is the total distance between your lists?", 
			"Its %d.", 
			part1.Resolve,
		),
		2: shared.CreateResolver(
			2,
			"What is their similarity score?", 
			"Its %d.", 
			part2.Resolve,
		),
	}
}


// Solve command contains all resolvers
var cmd = cobra.Command{
	Use:   "solve",
	Short: "Solve answer for selected part.",
	Long:  "Runs code and gets answer for selected part.",
	PersistentPreRun: preRun,
}

var cfgFile string
var resolvers map[int]cobra.Command

func init() {
	rootCmd.AddCommand(&cmd)
	
	resolvers = createResolvers()
	for _, v  := range resolvers {
		cmd.AddCommand(&v)
	}
	
	cmd.PersistentFlags().StringVarP(&cfgFile, "config", "c", "", "config file (default is ./data/part_X.toml)")
}

// Loads correct config for resolver
func preRun(cmd *cobra.Command, args []string) {
	var part int
	for k, v  := range resolvers {
		if cmd.Name() == v.Name() {
			part = k
		}
	}
	shared.InitConfig(cfgFile, part)
}