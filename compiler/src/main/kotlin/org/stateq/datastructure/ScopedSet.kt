package org.stateq.datastructure

class ScopedSet<E> : Set<E> {

    private val stack: ArrayDeque<HashSet<E>> = ArrayDeque()

    fun enterScope(): Boolean = stack.add(HashSet())

    fun exitScope(): Set<E> = stack.removeLast()

    fun add(element: E): Boolean = stack.last().add(element)

    override val size: Int get() = stack.fold(0) { acc, set -> acc + set.size }

    override fun isEmpty(): Boolean = stack.isEmpty()

    override fun iterator(): Iterator<E> = stack.flatten().iterator()

    override fun contains(element: E): Boolean = stack.any { it.contains(element) }

    override fun containsAll(elements: Collection<E>): Boolean {
        return elements.all { element ->
            stack.any { set -> set.contains(element) }
        }
    }
}
