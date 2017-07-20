class PigLatin {
    companion object {
        private val vowelsPattern = Regex("([aeiou]|xr|yt).*", RegexOption.IGNORE_CASE)
        private val constantPattern = Regex("(s?ch|qu|[^aeiou]qu|thr?|[^aeiou])(.*)", RegexOption.IGNORE_CASE)

        fun translate(sequence: String) = sequence.split(' ').map {
            if (!vowelsPattern.matches(it)) {
                val groups = constantPattern.matchEntire(it)!!.groupValues
                "${groups[2]}${groups[1]}"
            } else it
        }.joinToString("ay ").let { if (it.isNotEmpty()) it.plus("ay") else it }
    }
}