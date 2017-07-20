class PhoneNumber(pn: String) {
    val number = pn.filter(Char::isDigit).let {
        val nums = if (it.length == 11 && it[0] == '1') it.drop(1) else it
        when {
            nums.length != 10 || nums[0] < '2' || nums[3] < '2' -> throw IllegalArgumentException()
            else -> nums
        }
    }

    val areaCode: String
        get() = number.take(3)

    override fun toString() = "($areaCode) ${number.drop(3).take(3)}-${number.drop(6)}"
}