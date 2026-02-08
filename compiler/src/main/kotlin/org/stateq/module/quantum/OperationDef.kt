package org.stateq.module.quantum

import org.stateq.intermediate.IntermediateFunctionBase
import org.stateq.intermediate.IntermediateOperation
import org.stateq.module.FunctionLikeDefinition
import org.stateq.module.Scope
import org.stateq.parameter.*
import org.stateq.statement.StatementsBlock
import org.stateq.util.Location

class OperationDef(
    val ident: String,
    val classicalParams: List<ClassicalVariable>,
    val quantumParams: List<QuantumParameter>,
    val body: StatementsBlock?,
    override val location: Location,
) : FunctionLikeDefinition, Scope {
    override fun transpile(): IntermediateFunctionBase {
        val hiddenParams = quantumParams[0].sizeInferenceVariable?.let { hiddenParam ->
            assert(quantumParams.size == 1)
            listOf(hiddenParam)
        } ?: listOf()
        return IntermediateOperation(
            ident, false, hiddenParams + classicalParams, quantumParams, body?.transpile()?.decompose()
        )
    }
}
