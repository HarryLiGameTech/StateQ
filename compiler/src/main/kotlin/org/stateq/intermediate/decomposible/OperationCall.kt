package org.stateq.intermediate.decomposible

import org.stateq.exception.SizeNotMatchException
import org.stateq.exception.unreachable
import org.stateq.expression.*
import org.stateq.intermediate.DecomposableInstruction
import org.stateq.intermediate.Instruction
import org.stateq.intermediate.decomposed.ElementaryOperationCall
import org.stateq.intermediate.decomposed.DecomposedBasicBlock
import org.stateq.parameter.IntVariable
import org.stateq.qubit.QubitAccessor
import org.stateq.util.Location

class OperationCall(
    val op: OperationExpr, val target: QubitAccessor,
    override val location: Location
) : DecomposableInstruction() {

    init {
        if (op.isSizeDetermined && op.size != target.size) {
            throw SizeNotMatchException(op, target.size, op.location)
        }
    }

    override val decomposed: DecomposedBasicBlock by lazy {
        when (val operation = this.op) {
            is OperationExprElementary -> {
                ElementaryOperationCall(operation, target, location).asDecomposedBasicBlock()
            }
            is OperationExprSequentialMatMul -> decomposeSequentialOperation(operation).decompose()
            is OperationExprCombined -> decomposeCombinedOperation(operation).decompose()
            is OperationExprExtended -> decomposeExtendedOperation(operation).decompose()
            is OperationExprDagger -> decomposeDaggerOperation(operation).decompose()
            else -> unreachable()
        }
    }

    private fun decomposeSequentialOperation(sequentialOperation: OperationExprSequentialMatMul): BasicBlock {
        return sequentialOperation.sequence.reversed().map {
            if (it.size == target.size) {
                OperationCall(it, target, it.location)
            } else {
                throw SizeNotMatchException(it, target.size, op.location)
            }
        }.let { BasicBlock(it) }
    }

    private fun decomposeCombinedOperation(combinedOperation: OperationExprCombined): BasicBlock {
        return mutableListOf<Instruction>().let { instructions ->
            var accumulator = IntExpr(0)
            combinedOperation.operations.forEach { op ->
                val start = accumulator
                accumulator += op.size
                val end = accumulator
                val accessor = target.slice(start, end)
                // accessor.declare()?.also(instructions::add)
                instructions.add(op applyTo accessor)
            }
            return@let instructions
        }.let { BasicBlock(it) }
    }

    private fun decomposeExtendedOperation(extendedOperation: OperationExprExtended): BasicBlock {
        val iterParam = IntVariable(
            "iterExtOp_${extendedOperation.hashCode().toString(16)}",
            extendedOperation.location
        )
        // U@n -> for loop
        return mutableListOf<Instruction>().also { instructions ->
            val innerOpSize = extendedOperation.innerSize
            val accessor = if (innerOpSize == IntExpr(1)) {
                // e.g. `H@n psi` -> `for i in [0 til n] { H(psi[i]); }`
                this.target[iterParam.expr()]
            } else {
                // e.g. `SWAP@(n/2) psi` -> `for i in [0 til n/2] { SWAP(psi[i*2 : (i+1)*2]); }`
                this.target.slice(innerOpSize * iterParam.expr(), innerOpSize * (iterParam.expr() + 1))
            }
            // accessor.declare()?.also(instructions::add)
            instructions.add(extendedOperation.operation applyTo accessor)
        }.let {
            val range = IntExpr(0) until extendedOperation.multiplier
            BasicBlock(it).loopFor(iterParam, range, extendedOperation.location).asBasicBlock()
        }
    }

    private fun decomposeDaggerOperation(daggerOperation: OperationExprDagger): BasicBlock {
        return BasicBlock(
            DaggerBlock(
                OperationCall(daggerOperation.operation, target, daggerOperation.location).asBasicBlock(),
                daggerOperation.location,
            )
        )
    }

    companion object {
        infix fun OperationExpr.applyTo(target: QubitAccessor) = OperationCall(this, target, this.location)
    }
}
