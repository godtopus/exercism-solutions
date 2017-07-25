data class ComplexNumber(val real: Double = 0.0, val imag: Double = 0.0) {
    val abs: Double = Math.sqrt(real * real + imag * imag)
    
    operator fun plus(other: ComplexNumber) =
            ComplexNumber(real + other.real, imag + other.imag)
    
    operator fun minus(other: ComplexNumber) =
            ComplexNumber(real - other.real, imag - other.imag)
    
    operator fun times(other: ComplexNumber) =
            ComplexNumber(real * other.real - imag * other.imag, imag * other.real + real * other.imag)
    
    operator fun div(other: ComplexNumber) =
            ComplexNumber((real * other.real + imag * other.imag) / (other.real * other.real + other.imag * other.imag),
                    (imag * other.real - real * other.imag) / (other.real * other.real + other.imag * other.imag))

    operator fun times(factor: Double) =
            ComplexNumber(factor * real, factor * imag)
    
    fun conjugate() = ComplexNumber(real, -imag)
}

fun exponential(exponent: ComplexNumber) =
        ComplexNumber(Math.cos(exponent.imag), Math.sin(exponent.imag)) * Math.exp(exponent.real)