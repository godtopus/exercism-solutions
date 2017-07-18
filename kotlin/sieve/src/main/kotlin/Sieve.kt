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

class Sieve {
    companion object {
        fun primesUpTo(n: Int): List<Int> {
            var found = mutableListOf<Int>()

            var primes: Iterator<Int> = NaturalsGreaterThan(1)

            while (true) {
                val prime = primes.next()
                if (prime > n) {
                    return found
                }

                found.add(prime)
                primes = ExcludingMultiples(prime, primes)
            }
        }
    }
}