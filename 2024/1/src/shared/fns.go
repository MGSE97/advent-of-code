package shared

func Abs[T int | int8 | int16 | int32 | int64 | float32 | float64](x T) T {
	return If(x < 0, -x, x)
}