class ETL {
    companion object {
        fun transform(old: Map<Int, List<Char>>) = old.flatMap { val key = it.key; it.value.map { it.toLowerCase() }.zip(List(it.value.size) { key }) }.toMap()
    }
}