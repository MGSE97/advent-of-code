package shared

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

func ReadFile(key string) string {
	wd, err := os.Getwd()
	cobra.CheckErr(err)

	file := filepath.Join(wd, "data", viper.GetString(key))
	fmt.Printf("Using file '%s' from %s\n", key, file)

	dat, err := os.ReadFile(file)
    cobra.CheckErr(err)
    return string(dat)
}