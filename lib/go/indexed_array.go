package aoc

type IndexedArray[K comparable, V interface{}] struct {
	Ind          map[K]int
	Data         []V
	defaultValue V
	defaultFunc  func(K) V
}

func NewIndexedArray[K comparable, V interface{}](size int) IndexedArray[K, V] {
	return IndexedArray[K, V]{
		Ind:         make(map[K]int),
		Data:        make([]V, 0, size),
		defaultFunc: nil,
	}
}

func NewIndexedArrayFunc[K comparable, V interface{}](size int, defaultFunc func(K) V) IndexedArray[K, V] {
	return IndexedArray[K, V]{
		Ind:         make(map[K]int),
		Data:        make([]V, 0, size),
		defaultFunc: defaultFunc,
	}
}

func (a *IndexedArray[K, V]) GetOrCreate(index K) (int, V) {
	if i, ok := a.Ind[index]; ok {
		return i, a.Data[i]
	}
	i := len(a.Data)
	if a.defaultFunc != nil {
		a.Data = append(a.Data, a.defaultFunc(index))
	} else {
		a.Data = append(a.Data, a.defaultValue)
	}
	a.Ind[index] = i
	return i, a.Data[i]
}
