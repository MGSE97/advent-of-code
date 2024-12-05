package solve

import (
	"fmt"
	"os"
	"path"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
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

func init() {
}

// initConfig reads in config file and ENV variables if set.
func initConfig(file string, part int) {
	// Find working directory.
	wd, err := os.Getwd()
	cobra.CheckErr(err)

	// Set config search path and type
	viper.AddConfigPath(wd)
	viper.AddConfigPath(path.Join(wd, "data"))
	viper.SetConfigType("toml")

	var fileName string = file
	if file != "" {
		// Use config file from the flag.
		viper.SetConfigFile(file)
	} else {
		// Search config in working directory with name "part_{part}" (without extension).
		fileName = fmt.Sprintf("part_%d", part)
		viper.SetConfigName(fileName)
	}

	viper.AutomaticEnv() // read in environment variables that match

	// If a config file is found, read it in.
	cerr := viper.ReadInConfig()
	if cerr == nil {
		fmt.Fprintln(os.Stderr, "Using config file:", viper.ConfigFileUsed())
	} else {
		fmt.Fprintln(os.Stderr, "Missing config file:", fmt.Sprintf("%s.toml", fileName))
		cobra.CheckErr(cerr)
	}
}
