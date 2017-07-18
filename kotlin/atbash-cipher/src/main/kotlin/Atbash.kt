fun <T> Sequence<T>.batch(n: Int): Sequence<Sequence<T>> {
    return BatchingSequence(this, n)
}

private class BatchingSequence<T>(val source: Sequence<T>, val batchSize: Int) : Sequence<Sequence<T>> {
    override fun iterator(): Iterator<Sequence<T>> = object : AbstractIterator<Sequence<T>>() {
        val iterate = if (batchSize > 0) source.iterator() else emptyList<T>().iterator()

        override fun computeNext() {
            if (iterate.hasNext()) setNext(iterate.asSequence().take(batchSize))
            else done()
        }
    }
}

class Atbash {
    companion object {
        private val codes = ('a'..'z').zip('z' downTo 'a').toMap()

        fun encode(message: String): String {
            return message.filter(Char::isLetterOrDigit)
                          .toLowerCase()
                          .map { codes.getOrDefault(it, it) }
                          .asSequence()
                          .batch(5)
                          .map { it.joinToString(separator = "") }
                          .joinToString(separator = " ")
        }

        fun decode(cipherText: String) = cipherText.filter(Char::isLetterOrDigit).toLowerCase().map { codes.getOrDefault(it, it) }.joinToString(separator = "")
    }
}