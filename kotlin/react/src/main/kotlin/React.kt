class Reactor<T> {
    abstract inner class Cell {
        abstract val value: T
        internal val dependents = mutableListOf<ComputeCell>()
    }

    interface Subscription {
        fun cancel()
    }

    inner class InputCell(initialValue: T) : Cell() {
        override var value: T = initialValue
            set(newValue) {
                field = newValue
                dependents.forEach { it.propagate() }
                dependents.forEach { it.fireCallbacks() }
            }
    }

    inner class ComputeCell private constructor(val newValue: () -> T) : Cell() {
        override var value: T = newValue()
            private set

        private var lastCallbackValue = value
        private var callbacksIssued = 0
        private val activeCallbacks = mutableMapOf<Int, (T) -> Any>()

        constructor(vararg cells: Cell, f: (List<T>) -> T) : this({ f(cells.map { it.value }) }) {
            cells.forEach { it.dependents.add(this) }
        }

        fun addCallback(f: (T) -> Any): Subscription {
            val id = callbacksIssued++
            activeCallbacks[id] = f

            return object : Subscription {
                override fun cancel() {
                    activeCallbacks.remove(id)
                }
            }
        }

        internal fun propagate() {
            val nv = newValue()
            if (nv == value) return

            value = nv
            dependents.forEach { it.propagate() }
        }

        internal fun fireCallbacks() {
            if (value == lastCallbackValue) return

            lastCallbackValue = value
            activeCallbacks.values.forEach { it(value) }

            dependents.forEach { it.fireCallbacks() }
        }
    }
}