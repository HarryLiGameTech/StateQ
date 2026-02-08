package org.stateq.expression

import org.stateq.intermediate.decomposible.BasicBlock
import org.stateq.intermediate.decomposible.OperationCall
import org.stateq.parameter.QvarVariable
import org.stateq.qubit.*
import org.stateq.util.BitArray
import org.stateq.util.Locatable
import org.stateq.util.Location
import org.stateq.util.Range

abstract class QvarExpr(override val location: Location) : Locatable, QuantumExpr {

    abstract val size: IntExpr

    fun slice(
        start: IntExpr?, end: IntExpr?, step: IntExpr = IntExpr(1),
        inclusive: Boolean = false, location: Location? = null
    ) = QvarExprSlice(this, start, end, step, inclusive, location)

    abstract fun toQubitAccessorWithBasicBlock(): Pair<QubitAccessor, BasicBlock>

    fun toBasicBlock() = this.toQubitAccessorWithBasicBlock().second
}

class QvarExprVariable(val variable: QvarVariable, location: Location) : QvarExpr(location) {

    override val size: IntExpr get() = variable.size

    override fun toQubitAccessorWithBasicBlock(): Pair<QubitAccessor, BasicBlock> {
        return Pair(QubitAccessorVariable(this.variable), BasicBlock())
    }
}

abstract class QvarExprInit(location: Location) : QvarExpr(location) {
    companion object {
        fun staticInit(size: UInt, location: Location) = QvarExprInitStatic(size, location)
        fun encode(value: UInt, location: Location) = QvarExprInitStatic(BitArray(value), location)
        fun encode(value: UInt, size: UInt, location: Location) = QvarExprInitStatic(size, value, location)
        fun encode(value: IntExpr, size: IntExpr, location: Location) = QvarExprInitDynamic(size, value, location)
        fun encode(value: IntExpr, size: UInt, location: Location) = QvarExprInitDynamic(IntExpr(size.toInt()), value, location)
        fun encode(value: UInt, size: IntExpr, location: Location) = QvarExprInitDynamic(size, IntExpr(value.toInt()), location)
    }
}

class QvarExprInitStatic(
    private val encodeData: BitArray, location: Location
) : QvarExprInit(location) {

    override val size: IntExpr = IntExpr(encodeData.size())

    constructor(size: UInt, location: Location): this(BitArray.zero(size), location)
    constructor(size: UInt, value: UInt, location: Location): this(BitArray(value, size), location)

    override fun toQubitAccessorWithBasicBlock(): Pair<QubitAccessor, BasicBlock> {
        val accessor = QubitAccessorAlloc(this.size)
        return Pair(accessor, BasicBlock(accessor.encode(this.encodeData.toInt())))
    }
}

class QvarExprInitDynamic(
    override val size: IntExpr, private val encodeData: IntExpr, location: Location
) : QvarExprInit(location) {

    init {
        assert(!size.isZero())
    }

    override fun toQubitAccessorWithBasicBlock(): Pair<QubitAccessor, BasicBlock> {
        val accessor = QubitAccessorAlloc(this.size)
        return Pair(accessor, BasicBlock(accessor.encode(this.encodeData)))
    }
}

class QvarExprIndexing(
    val inner: QvarExpr, val index: IntExpr,
    location: Location? = null,
) : QvarExpr(location ?: inner.location) {

    override val size = IntExpr(1)

    override fun toQubitAccessorWithBasicBlock(): Pair<QubitAccessor, BasicBlock> {
        return inner.toQubitAccessorWithBasicBlock().let { (target, block) ->
            Pair(QubitAccessorIndexing(target, index), block)
        }
    }
}

class QvarExprSlice(
    val inner: QvarExpr,
    start: IntExpr?, end: IntExpr?, step: IntExpr?,
    val inclusive: Boolean,
    location: Location? = null,
) : QvarExpr(location ?: inner.location) {

    constructor(inner: QvarExpr, range: Range, location: Location?) : this(
        inner, range.start, range.end, range.step, range.inclusive, location
    )

    val start = start ?: IntExpr(0)
    val end by lazy { (end ?: inner.size) - if (inclusive) 1 else 0 }
    val step = step ?: IntExpr(1)

    override val size by lazy {
        ((this.end - this.start) + if (inclusive) 1 else 0) / this.step
    }

    override fun toQubitAccessorWithBasicBlock(): Pair<QubitAccessor, BasicBlock> {
        return inner.toQubitAccessorWithBasicBlock().let { (target, block) ->
            Pair(QubitAccessorSlicing(target, start, end, step, inclusive), block)
        }
    }
}

class QvarExprConcat(val exprs: List<QvarExpr>, location: Location) : QvarExpr(location) {

    override val size: IntExpr by lazy {
        exprs.fold(IntExpr(0)) { acc, expr -> acc + expr.size }
    }

    override fun toQubitAccessorWithBasicBlock(): Pair<QubitAccessor, BasicBlock> {
        return this.exprs.map(QvarExpr::toQubitAccessorWithBasicBlock).let {
            val block = it.fold(BasicBlock()) { acc, (_, block) -> acc + block }
            Pair(QubitAccessorConcat(it.map { (accessor, _) -> accessor }), block)
        }
    }
}

class QvarExprOperation(
    val operation: OperationExpr, val target: QvarExpr, location: Location
) : QvarExpr(location) {

    override val size: IntExpr by lazy { target.size }

    override fun toQubitAccessorWithBasicBlock(): Pair<QubitAccessor, BasicBlock> {
        return this.target.toQubitAccessorWithBasicBlock().let { (accessor, block) ->
            Pair(accessor, block + BasicBlock(OperationCall(operation, accessor, location)))
        }
    }
}

class QvarExprOperationMultiTargets(
    val operation: OperationExprElementary, val targets: List<QvarExpr>, location: Location
) : QvarExpr(location) {

    override val size: IntExpr by lazy {
        targets.fold(IntExpr(0)) { acc, target -> acc + target.size }
    }

    override fun toQubitAccessorWithBasicBlock(): Pair<QubitAccessor, BasicBlock> {
        TODO("Not yet implemented")
    }
}
