fun <T> List<T>.windows(windowSize: Int): List<List<T>> {
    return this.dropLast(windowSize - 1).mapIndexed { i, _ -> this.subList(i, i + windowSize) }
}

class Series {
    companion object {
        fun slices(length: Int, sequence: String) = sequence.map { it.toInt() - 48 }.windows(length)
    }
}