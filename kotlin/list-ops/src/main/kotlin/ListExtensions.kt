fun <T> List<T>.customAppend(values: List<T>) = listOf(this, values).flatMap { it }

fun <T> List<List<T>>.customConcat() = flatMap { it }

fun <T> List<T>.customFilter(predicate: (T) -> Boolean): List<T> {
    val result = mutableListOf<T>()
    forEach { if (predicate(it)) { result.add(it) } }
    return result
}

val List<Any>.customSize: Int
    get() = size

fun <T, U> List<T>.customMap(transform: (T) -> U): List<U> {
    val result = mutableListOf<U>()
    forEach { result.add(transform(it)) }
    return result
}

fun <T, U> List<T>.customFoldLeft(initial: U, f: (U, T) -> U): U {
    if (isEmpty()) return initial
    return drop(1).customFoldLeft(f(initial, first()), f)
}

fun <T, U> List<T>.customFoldRight(initial: U, f: (T, U) -> U): U {
    if (isEmpty()) return initial
    return f(first(), drop(1).customFoldRight(initial, f))
}

fun <T> List<T>.customReverse(): List<T> = (size - 1 downTo 0).map { this[it] }