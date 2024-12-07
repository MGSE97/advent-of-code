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

	name := viper.GetString(key)
	file := filepath.Join(wd, "data", name)
	fmt.Printf("Reading file '%s' from '%s'\n", key, name)

	dat, err := os.ReadFile(file)
    cobra.CheckErr(err)
    return string(dat)
}