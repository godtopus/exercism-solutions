class DNA(sequence: String) {
    var nucleotideCounts = sequence.let {
        if (sequence.any { !valid(it) }) throw IllegalArgumentException()

        var counts = mutableMapOf('A' to 0, 'C' to 0, 'G' to 0, 'T' to 0)
        sequence.map { counts[it] = counts.getValue(it) + 1 }
        counts
    }

    fun count(nucleotide: Char) = if (!valid(nucleotide)) throw IllegalArgumentException() else nucleotideCounts.getOrDefault(nucleotide, 0)

    private fun valid(c: Char) = c == 'A' || c == 'C' || c == 'G' || c == 'T'
}