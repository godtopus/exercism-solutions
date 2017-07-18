class NaturalsGreaterThan(n: Int): Iterator<Int> {
    var v = n + 1

    override fun hasNext() = true
    override fun next(): Int = v++
}

class ExcludingMultiples(val n: Int, val base: Iterator<Int>): Iterator<Int> {
    override fun hasNext() = true

    override fun next(): Int {
        while (true) {
            val i = base.next()
            if (i % n != 0) {
                return i
            }
        }
    }
}

class Prime {
    companion object {
        fun nth(n: Int): Int {
            if (n < 1) {
                throw IllegalArgumentException()
            }

            var primes: Iterator<Int> = NaturalsGreaterThan(1)

            return (0..n - 1).map { val prime = primes.next(); primes = ExcludingMultiples(prime, primes); prime }.last()
        }
    }
}