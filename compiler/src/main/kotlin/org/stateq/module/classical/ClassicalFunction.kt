package org.stateq.module.classical

import org.stateq.expression.ClassicalExpr
import org.stateq.module.Definition
import org.stateq.util.Location
import org.stateq.type.ClassicalType
import org.stateq.type.matches
import org.stateq.util.CompileError

abstract class ClassicalFunction<out E: ClassicalExpr>(val ident: String): Definition {

    abstract val paramTypes: List<ClassicalType>

    protected abstract fun callInner(args: List<ClassicalExpr>, location: Location): E

    fun call(args: List<ClassicalExpr>, location: Location): E {
        if (args.size != paramTypes.size) {
            val adj = if (args.size < paramTypes.size) "few" else "many"
            CompileError.error(location,
                "Too $adj arguments to function call `$ident`, " +
                "expected ${paramTypes.size} found ${args.size}"
            ).raise()
        }
        paramTypes.zip(args).forEach { (type, expr) ->
            if (!(expr matches type)) {
                CompileError.error(location,
                    "Incompatible parameter, expected $type found ${expr.type}"
                ).raise()
            }
        }
        return this.callInner(args, location)
    }
}
