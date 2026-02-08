package org.stateq.qubit

import org.stateq.exception.SizeNotMatchException
import org.stateq.expression.IntExpr
import org.stateq.expression.OperationExpr
import org.stateq.intermediate.*
import org.stateq.intermediate.decomposible.OperationCall.Companion.applyTo
import org.stateq.intermediate.decomposible.BasicBlock
import org.stateq.intermediate.decomposible.ForLoop
import org.stateq.intermediate.decomposible.loopFor
import org.stateq.intermediate.decomposed.BeginControl
import org.stateq.intermediate.decomposed.EndControl
import org.stateq.parameter.IntVariable
import org.stateq.type.IntTrait

fun QubitAccessor.forEachQubitApply(operation: OperationExpr): ForLoop<IntTrait> {
    if (operation.size != IntExpr(1)) {
        throw SizeNotMatchException(operation, IntExpr(1), operation.location)
    }
    val iterParamIdent = "forQubitIter_${(operation.hashCode() * 131 + this.hashcode).toString(16)}"
    val iterParam = IntVariable(iterParamIdent, operation.location)
    return mutableListOf<Instruction>().also { instructions ->
        val accessor = this[iterParam.expr()]
        // accessor.declare().also(instructions::add)
        instructions.add(operation applyTo accessor)
    }.let {
        BasicBlock(it).loopFor(iterParam, IntExpr(0) .. this.size, operation.location)
    }
}

fun QubitAccessor.beginControl(condition: Boolean = true) = BeginControl(this, condition, null)
fun QubitAccessor.endControl() = EndControl(this, null)
