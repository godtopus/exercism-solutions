class SumOfMultiples {
    companion object {
        fun sum(factors: Set<Int>, n: Int) = (1..n - 1).filter { i -> factors.any { i % it == 0 } }.sum()
    }
}