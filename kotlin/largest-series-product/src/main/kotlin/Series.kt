class Series(sequence: String) {
    var sequence = sequence

    init {
        if (!sequence.all { it.isDigit() }) {
            throw IllegalArgumentException()
        }
    }

    fun getLargestProduct(n: Int): Long {
        if (n > sequence.length || n < 0) {
            throw IllegalArgumentException()
        }

        if (n == 0) {
            return 1
        }

        var numberSequence = sequence.map { it.toLong() - 48 }

        return (0..numberSequence.count() - n).map { numberSequence.subList(it, it + n).fold(1L, { total, next -> total * next }) }.max()!!
    }
}