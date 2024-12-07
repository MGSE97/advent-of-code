package shared

import (
	"fmt"
	"os"
	"path"
	"path/filepath"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

// Globally accesible current configuration
var Config RunConfig

// Configuration structure
// Change it based on usecase
type RunConfig struct {
	Input string `mapstructure:"input"`
}

// InitConfig reads in config file and ENV variables if set.
func InitConfig(file string, part int) {
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
		viper.SetConfigName(file)
	} else {
		// Search config in working directory with name "part_{part}" (without extension).
		fileName = fmt.Sprintf("part_%d", part)
		viper.SetConfigName(fileName)
	}

	viper.AutomaticEnv() // read in environment variables that match

	// If a config file is found, read it into memory.
	cerr := viper.ReadInConfig()
	if cerr == nil {
		fmt.Fprintln(os.Stdout, "Using config file:", filepath.Base(viper.ConfigFileUsed()))
	} else {
		fmt.Fprintln(os.Stderr, "Missing config file:", fmt.Sprintf("%s.toml", fileName))
		cobra.CheckErr(cerr)
	}

	// Bind config map into structure
	uerr := viper.Unmarshal(&Config);
	cobra.CheckErr(uerr)
}