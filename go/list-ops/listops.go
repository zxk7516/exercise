package listops

type binFunc func(x, y int) int
type predFunc func(n int) bool
type unaryFunc func(n int) int

type IntList []int

func (this IntList) Length() int {
	return len(this)
}

func (this IntList) Reverse() IntList {
	for i, j := 0, len(this)-1; i < j; i, j = i+1, j-1 {
		this[i], this[j] = this[j], this[i]
	}
	return this
}

func (this IntList) Map(f unaryFunc) IntList {
	for i, _ := range this {
		this[i] = f(this[i])
	}
	return this
}
func (this IntList) Append(ot IntList) IntList {
	for _, v := range ot {
		this = append(this, v)
	}
	return this
}

func (this IntList) Foldl(f binFunc, initial int) int {
	for i := 0; i < len(this); i++ {
		initial = f(initial, this[i])
	}
	return initial
}

func (this IntList) Foldr(f binFunc, initial int) int {
	for i := len(this) - 1; i >= 0; i-- {
		initial = f(this[i], initial)
	}
	return initial
}

func (this IntList) Filter(fn predFunc) IntList {
	ret := make(IntList, 0)
	for _, v := range this {
		if fn(v) {
			ret = append(ret, v)
		}
	}
	return ret
}

func (this IntList) Concat(lists []IntList) IntList {
	ret := this
	for _, ll := range lists {
		for _, llItem := range ll {
			ret = append(ret, llItem)
		}
	}
	return ret
}
