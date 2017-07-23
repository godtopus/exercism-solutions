class BracketPush {
    companion object {
        fun isValid(sequence: String): Boolean {
            var openStack = mutableListOf<Char>()

            sequence.map {
                when (it) {
                    '(' -> openStack.add(')')
                    '{' -> openStack.add('}')
                    '[' -> openStack.add(']')
                    ')', '}', ']' -> if (openStack.size > 0 && openStack.removeAt(openStack.size - 1) == it) {} else return false
                    else -> {}
                }
            }

            return openStack.isEmpty()
        }
    }
}