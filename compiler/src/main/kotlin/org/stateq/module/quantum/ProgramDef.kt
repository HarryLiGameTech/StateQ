package org.stateq.module.quantum

import org.stateq.expression.IntExpr
import org.stateq.intermediate.IntermediateFunctionBase
import org.stateq.intermediate.IntermediateProgram
import org.stateq.module.FunctionLikeDefinition
import org.stateq.module.Scope
import org.stateq.parameter.ClassicalVariable
import org.stateq.statement.StatementsBlock
import org.stateq.util.Location

class ProgramDef(
    val ident: String,
    val params: List<ClassicalVariable>,
    val shots: IntExpr,
    val body: StatementsBlock,
    override val location: Location,
) : FunctionLikeDefinition, Scope {
    override fun transpile(): IntermediateFunctionBase {
        return IntermediateProgram(ident, params, shots, body.transpile().decompose())
    }
}
