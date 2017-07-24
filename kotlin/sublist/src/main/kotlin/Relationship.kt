fun <T> List<T>.windows(windowSize: Int): List<List<T>> {
    return this.dropLast(windowSize - 1).mapIndexed { i, _ -> this.subList(i, i + windowSize) }
}

enum class Relationship {

    EQUAL, SUBLIST, SUPERLIST, UNEQUAL

}

fun <T> List<T>.relationshipTo(other: List<T>): Relationship {
    return if (size <= other.size) {
        findRelationship(other)
    } else {
        val relationship = other.findRelationship(this)
        when (relationship) {
            Relationship.SUBLIST -> Relationship.SUPERLIST
            else -> relationship
        }
    }
}

private fun <T> List<T>.findRelationship(other: List<T>): Relationship {
    if (isEmpty() && other.size > 0) {
        return Relationship.SUBLIST
    } else if (isEmpty() && other.isEmpty()) {
        return Relationship.EQUAL
    }

    val matchFound = other.windows(size)
                          .map { it.zip(this).dropWhile { it.first == it.second }.count() == 0 }
                          .any { it }

    return if (matchFound) {
        if (size == other.size) {
            Relationship.EQUAL
        } else {
            Relationship.SUBLIST
        }
    } else {
        Relationship.UNEQUAL
    }
}