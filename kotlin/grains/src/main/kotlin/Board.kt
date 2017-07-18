import java.math.BigInteger

class Board {
    companion object {
        fun getGrainCountForSquare(n: Int): BigInteger {
            if (n < 1 || n > 64) {
                throw IllegalArgumentException("Only integers between 1 and 64 (inclusive) are allowed")
            }

            return BigInteger.valueOf(2).pow(n - 1)
        }

        fun getTotalGrainCount() = BigInteger.valueOf(2).pow(64).subtract(BigInteger.ONE)
    }
}