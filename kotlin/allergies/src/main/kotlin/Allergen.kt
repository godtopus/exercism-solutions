enum class Allergen(val score: Int) {
    EGGS(1),
    PEANUTS(2),
    SHELLFISH(4),
    STRAWBERRIES(8),
    TOMATOES(16),
    CHOCOLATE(32),
    POLLEN(64),
    CATS(128)
}

class Allergies(val score: Int) {
    fun isAllergicTo(allergen: Allergen) = score and allergen.score == allergen.score

    fun getList() = listOf(Allergen.EGGS, Allergen.PEANUTS, Allergen.SHELLFISH, Allergen.STRAWBERRIES, Allergen.TOMATOES, Allergen.CHOCOLATE, Allergen.POLLEN, Allergen.CATS).filter { isAllergicTo(it) }
}