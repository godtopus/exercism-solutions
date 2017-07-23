class PrimeFactorCalculator {
    companion object {
        fun primeFactors(n: Int): List<Int> {
            return primeFactors(n.toLong()).map(Long::toInt)
        }

        fun primeFactors(n: Long): List<Long> {
            val factors = mutableListOf<Long>()
            var remainder = n
            var divisor = 2L

            while (remainder > 1) {
                while (remainder % divisor == 0L) {
                    factors.add(divisor)
                    remainder /= divisor
                }

                divisor++
            }

            return factors
        }
    }
}