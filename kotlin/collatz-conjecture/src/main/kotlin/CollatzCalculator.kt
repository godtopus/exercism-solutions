class CollatzCalculator {
    companion object {
        fun computeStepCount(n: Int): Int {
            if (n < 1) {
                throw IllegalArgumentException("Only natural numbers are allowed")
            }

            var steps = 0
            var current = n

            while (current != 1) {
                if (current % 2 == 0) {
                    current /= 2
                } else {
                    current = current * 3 + 1
                }

                steps++
            }

            return steps
        }
    }
}