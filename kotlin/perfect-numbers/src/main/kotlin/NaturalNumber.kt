
enum class Classification {
    DEFICIENT, PERFECT, ABUNDANT
}

fun classify(naturalNumber: Int): Classification {
    if (naturalNumber < 1) {
        throw RuntimeException()
    }

    var aliquotSum = (1..(naturalNumber / 2) + 1).filter { naturalNumber % it == 0 }.sum()
    return if (aliquotSum == naturalNumber) Classification.PERFECT else if (aliquotSum > naturalNumber) Classification.ABUNDANT else Classification.DEFICIENT
}
