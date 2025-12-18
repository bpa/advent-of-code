package aoc

func Reduce[T interface{}](arr []T, f func(T, T) T) T {
	return Fold(arr[0], arr[1:], f)
}

func Fold[T interface{}](acc T, arr []T, f func(T, T) T) T {
	for _, v := range arr {
		acc = f(acc, v)
	}
	return acc
}
