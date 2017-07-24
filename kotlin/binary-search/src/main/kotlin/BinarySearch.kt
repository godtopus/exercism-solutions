class BinarySearch {
    companion object {
        fun search(values: List<Int>, key: Int): Int {
            require(values == values.sorted())

            var found = -1
            var start = 0
            var end = values.count()

            while (start < end) {
                val midIndex = start + (end - start) / 2

                if (values[midIndex] == key) {
                    found = midIndex
                    break
                } else if (values[midIndex] < key) {
                    start = midIndex + 1
                } else {
                    end = midIndex
                }
            }

            return found
        }
    }
}