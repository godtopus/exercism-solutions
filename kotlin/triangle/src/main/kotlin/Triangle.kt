class Triangle(a: Double, b: Double, c: Double) {
    var isEquilateral = false
    var isIsosceles = false
    var isScalene = false

    init {
        if (!(a > 0 && b > 0 && c > 0) || a + b < c || a + c < b || b + c < a) {
            throw IllegalArgumentException()
        }

        this.isEquilateral = a == b && b == c
        this.isIsosceles = a == b || a == c || b == c
        this.isScalene = a != b && a != c && b != c
    }

    constructor(a: Int, b: Int, c: Int) : this(a.toDouble(), b.toDouble(), c.toDouble())
}