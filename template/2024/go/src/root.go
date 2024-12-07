package solve

import (
	"os"

	"github.com/spf13/cobra"
)

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "lets",
	Short: "CLI application for solving Advent of code questions.",
	Long: `CLI application for solving Advent of code questions. 
	Written in Go, using Cobra and Viper.
	For more details, visit https://github.com/MGSE97/advent-of-code`,
}

// Execute adds all child commands to the root command and sets flags appropriately.
// This is called by main.main(). It only needs to happen once to the rootCmd.
func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}
