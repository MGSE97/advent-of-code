package part2

import (
	"fmt"
	"lets/src/shared"
	"strings"

	"github.com/spf13/cobra"
)

func Resolve(cmd *cobra.Command, args []string) int {
	// Read file input
	input := shared.ReadFile("input")
	
	similarity := make(map[int]int)
	var listA []int
	var a int
	var b int
	
	// Parse file
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		n, err := fmt.Sscanf(line, "%d %d", &a, &b)
		cobra.CheckErr(err)
		if n > 0 {
			listA = append(listA, a)
			
			// Compute similarity for each number
			val, ok := similarity[b]
			if ok {
				similarity[b] = val + 1
			} else {
				similarity[b] = 1
			}
		}
	}

	// Compute distance
	var distance int = 0
	for _, a := range listA {
		b = similarity[a]
		distance += a * b
	}

	return distance
}