package org.stateq.intermediate.decomposed

import org.stateq.gates.StandardGate
import org.stateq.compiler.CodeGenerator
import org.stateq.exception.unreachable
import org.stateq.expression.OperationExprElementary
import org.stateq.expression.OperationExprStandard
import org.stateq.expression.OperationExprUserDefined
import org.stateq.intermediate.DecomposedInstruction
import org.stateq.qubit.QubitAccessor
import org.stateq.util.Location
import org.stateq.util.raiseCompileError

class ElementaryOperationCall(
    val op: OperationExprElementary, val target: QubitAccessor,
    override val location: Location
) : DecomposedInstruction {

    private fun getStandardGate(): StandardGate? {
        return try {
            StandardGate.valueOf(op.ident)
        } catch (_: IllegalArgumentException) {
            null
        }
    }

    override fun emit(codegen: CodeGenerator) {
        when (this.op) {
            is OperationExprStandard -> {
                if (this.op.gate != StandardGate.I) {
                    codegen.pushStdBuiltinOp(op.gate, op.classicalArgs, target)
                }
            }
            is OperationExprUserDefined -> {
                val firstParam = this.op.definition.quantumParams[0]
                val classicalArgs = firstParam.sizeInferenceVariable?.let {
                    assert(this.op.definition.quantumParams.size == 1)
                    try {
                        firstParam.size.solve(this.target.size, it).let { hiddenArgument ->
                            listOf(hiddenArgument) + this.op.classicalArgs
                        }
                    } catch (_: IllegalArgumentException) {
                        raiseCompileError(location) {
                            "Unable to solve quantum parameter size inference expression"
                        }
                    }
                } ?: this.op.classicalArgs
                codegen.operationCall(this.op, classicalArgs, listOf(this.target))
            }
            else -> unreachable()
        }
    }
}
