class Flattener {
    companion object {
        fun <T> flatten(sequence: List<T>): List<Any> {
            return sequence.flatMap {
                when (it) {
                    null -> listOf<Any>()
                    is List<*> -> flatten(it)
                    else -> listOf<Any>(it)
                }
            }
        }
    }
}