package part1

import (
	"fmt"
	"lets/src/shared"
	"slices"
	"strings"

	"github.com/spf13/cobra"
)

func Resolve(cmd *cobra.Command, args []string) int {
	// Read file input
	input := shared.ReadFile("input")
	
	var listA []int
	var listB []int
	var a int
	var b int
	
	// Parse file
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		n, err := fmt.Sscanf(line, "%d %d", &a, &b)
		cobra.CheckErr(err)
		if n > 0 {
			listA = append(listA, a)
			listB = append(listB, b)
		}
	}

	// Sort lists
	slices.Sort(listA)
	slices.Sort(listB)

	// Compute distance
	var distance int = 0
	for i, a := range listA {
		b = listB[i]
		distance += shared.Abs(a - b)
	}

	return distance
}