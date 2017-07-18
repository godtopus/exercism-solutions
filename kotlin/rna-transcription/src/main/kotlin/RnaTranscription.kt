fun transcribeToRna(dna: String): String {
    return dna.map {
        when (it) {
            'A' -> 'U'
            'C' -> 'G'
            'G' -> 'C'
            'T' -> 'A'
            else -> throw IllegalArgumentException("Not a valid DNA sequence")
        }
    }.joinToString("")
}
