package org.stateq.util

class ScopedIdentMap<V> : Map<String, V> {

    private val stack: ArrayDeque<HashMap<String, V>> = ArrayDeque()

    fun enterScope(): Boolean = stack.add(HashMap())

    fun exitScope(): Map<String, V> = stack.removeLast()

    override val entries get() = setOf<Map.Entry<String, V>>().let {
        stack.fold(it) { acc, map -> acc + map.entries }
    }

    override val keys get() = setOf<String>().let {
        stack.fold(it) { acc, map -> acc + map.keys }
    }

    override val values: Collection<V> = setOf<V>().let {
        stack.fold(it) { acc, map -> acc + map.values }
    }

    override val size: Int get() = (
        stack.fold(0) { acc, map -> acc + map.size }
    )

    val stackSize = stack.size

    override fun isEmpty(): Boolean = stack.isEmpty()

    override operator fun get(key: String): V? {
        stack.forEach {
            it[key]?.also { value -> return value }
        }
        return null
    }

    fun put(key: String, value: V): V? {
        this[key].let {
            if (it != null) {
                return it
            } else {
                this.stack.last()[key] = value
                return null
            }
        }
    }

    override fun containsValue(value: V): Boolean {
        return this.values.contains(value)
    }

    override fun containsKey(key: String): Boolean {
        return this[key] != null
    }
}
