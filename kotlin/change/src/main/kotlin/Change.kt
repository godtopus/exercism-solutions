class Change(private val coins: Set<Int>) {
    private val coinsSortedAscending = coins.sorted().toIntArray()

    fun issue(n: Int): List<Int> {
        require(n >= 0)

        if (n == 0) {
            return listOf()
        }

        require(n >= coinsSortedAscending.first())

        return issue(coinsSortedAscending, n)
    }

    private fun issue(coins: IntArray, target: Int): List<Int> {
        var table = IntArray(target + 1, { _ -> Int.MAX_VALUE })
        var result = IntArray(target + 1, { _ -> -1 })

        table[0] = 0

        (1..target).forEach { i ->
            (0..coins.size - 1).takeWhile { coins[it] <= i }.forEach { j ->
                if (table[i] > 1 + table[i - coins[j]]) {
                    table[i] = 1 + table[i - coins[j]]
                    result[i] = j
                }
            }
        }

        var change = mutableListOf<Int>()
        if (result[target] != -1) {
            var start = target
            while (start != 0) {
                val j = result[start]
                change.add(coins[j])
                start -= coins[j]
            }
        }

        return change
    }
}