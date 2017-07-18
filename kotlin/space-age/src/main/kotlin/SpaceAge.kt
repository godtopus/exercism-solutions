class SpaceAge(val seconds: Long) {
    companion object {
        private val earthSecondsPerYear = 31557600L
        private val mercuryYearEarthPeriod = 0.2408467
        private val venusYearEarthPeriod = 0.61519726
        private val marsYearEarthPeriod = 1.8808158
        private val jupiterYearEarthPeriod = 11.862615
        private val saturnYearEarthPeriod = 29.447498
        private val uranusYearEarthPeriod = 84.016846
        private val neptuneYearEarthPeriod = 164.79132

        private val significant = 100.0
        fun format(number: Double) = Math.round(number * significant).div(significant)
    }

    fun onEarth() = age(1.0)
    fun onMercury() = age(mercuryYearEarthPeriod)
    fun onVenus() = age(venusYearEarthPeriod)
    fun onMars() = age(marsYearEarthPeriod)
    fun onJupiter() = age(jupiterYearEarthPeriod)
    fun onSaturn() = age(saturnYearEarthPeriod)
    fun onUranus() = age(uranusYearEarthPeriod)
    fun onNeptune() = age(neptuneYearEarthPeriod)

    private fun age(factor: Double) = format(seconds.toDouble().div(factor * earthSecondsPerYear))
}