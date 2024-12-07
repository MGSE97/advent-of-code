package shared

func If[T comparable](condition bool, success T, fail T) T {
	if condition {
		return success
	} else {
		return fail
	}
}