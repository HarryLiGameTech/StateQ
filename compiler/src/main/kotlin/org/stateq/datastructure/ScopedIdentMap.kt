package org.stateq.datastructure

class ScopedIdentMap<V : Any> : Map<String, V> {

    private sealed interface ScopeFrame

    private object Barrier : ScopeFrame

    private inner class Scope : ScopeFrame {
        val table: MutableMap<String, V> = HashMap()
    }

    private val rootScope: Scope = Scope()
    private val scopeStack: ArrayDeque<ScopeFrame> = ArrayDeque()

    fun currentScope(): Map<String, V> {
        return scopeStack.takeWhile { it !is Barrier }.fold(rootScope.table.toMap()) {
            acc, map -> map.table()?.let { acc + it } ?: acc
        }
    }

    private fun <T> foldCurrentScope(init: T, action: (T, Map<String, V>) -> T): T {
        return scopeStack.takeWhile { it !is Barrier }.fold(init) {
            acc, map -> map.table()?.let { action(acc, it) } ?: acc
        }
    }

    private fun ScopedIdentMap<*>.Scope.table(): MutableMap<String, V> {
        @Suppress("UNCHECKED_CAST")
        return this.table as MutableMap<String, V>
    }

    private fun ScopeFrame.table(): MutableMap<String, V>? = when (this) {
        is ScopedIdentMap<*>.Scope -> this.table()
        is Barrier -> null
    }

    fun enterScope(): Boolean = scopeStack.add(Scope())

    fun enterScopeWithBarrier(): Boolean = scopeStack.add(Barrier) && enterScope()

    fun exitScope(): Map<String, V> {
        return try {
            scopeStack.removeLast().also {
                while (scopeStack.last() is Barrier) scopeStack.removeLast()
            }.table()!!.toMap()
        } catch (e: NoSuchElementException) {
            throw IllegalStateException("No scope to exit")
        }
    }

    override val entries: Set<Map.Entry<String, V>> get() = foldCurrentScope(emptySet()) {
        acc, map -> acc + map.entries
    }

    override val keys: Set<String> get() = foldCurrentScope(emptySet()) {
        acc, map -> acc + map.keys
    }

    override val values: Collection<V> get() = foldCurrentScope(emptyList()) {
        acc, map -> acc + map.values
    }

    override val size: Int get() = foldCurrentScope(0) { acc, map -> acc + map.size }

    val stackSize = scopeStack.count { it !is Barrier }

    override fun isEmpty(): Boolean = scopeStack.isEmpty()

    override operator fun get(key: String): V? {
        return rootScope.table[key] ?: run {
            scopeStack.asReversed().firstNotNullOfOrNull { it.table()?.get(key) }
        }
    }

    fun put(key: String, value: V): V? {
        return this[key]?.let { return it } ?: scopeStack.last().table()!!.put(key, value)
    }

    override fun containsValue(value: V): Boolean {
        return foldCurrentScope(false) { acc, map -> acc || map.containsValue(value) }
    }

    override fun containsKey(key: String): Boolean {
        return this[key] != null
    }

    fun <T> withInScope(action: () -> T) {
        this.enterScope()
        action()
        this.exitScope()
    }

    fun <T> withInIsolatedScope(action: () -> T) {
        this.enterScopeWithBarrier()
        action()
        this.exitScope()
    }

    operator fun set(ident: String, value: V) {
        put(ident, value)
    }
}
