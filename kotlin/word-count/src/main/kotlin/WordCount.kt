class WordCount {
    companion object {
        fun phrase(phrase: String) = phrase.split(" ").map { it -> it.toLowerCase().trim { c -> !c.isLetterOrDigit() } }.filterNot(String::isEmpty).groupingBy { it }.eachCount()
    }
}