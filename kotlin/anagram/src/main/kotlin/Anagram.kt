class Anagram(val word: String) {
    private val sorted = word.toLowerCase().split("").sorted()

    fun match(words: List<String>): List<String> = words.filterNot { it.toLowerCase() == word }.filter { it.toLowerCase().split("").sorted() == sorted }
}