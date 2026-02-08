package org.stateq.type

import org.jetbrains.kotlinx.multik.ndarray.complex.ComplexDouble
import org.jetbrains.kotlinx.multik.ndarray.data.NDArray
import org.stateq.expression.ClassicalExpr
import org.stateq.math.Matrix
import org.stateq.parameter.*
import org.stateq.util.Location
import java.lang.IllegalArgumentException
import kotlin.reflect.KClass

interface Type {
    override fun toString(): String
    override fun hashCode(): Int
}

enum class QuantumType : Type {
    QubitAccessor, ProgramContext;
    override fun toString() = this.name
}

interface ReturnType : Type

object MeasurementResultType : ReturnType {
    override fun toString() = "MeasurementResult"
    override fun hashCode() = super.hashCode()
}

interface ClassicalType : ReturnType {

    override fun toString(): String

    fun createVariable(ident: String, location: Location?): ClassicalVariable

    val listType get() = ClassicalListType(this)

    companion object {

        val Bool = ClassicalBasicType.Bool
        val Int = ClassicalBasicType.Int
        val Float = ClassicalBasicType.Float
        val Complex = ClassicalBasicType.Complex
        val Bits = ClassicalBasicType.Bits
        val Mat = ClassicalBasicType.Mat

        fun of(ident: String): ClassicalType? {
            return try {
                ClassicalBasicType.valueOf(ident)
            } catch (_: IllegalArgumentException) {
                if (ident.startsWith("[") && ident.endsWith("]")) {
                    this.of(ident.drop(1).dropLast(1))?.listType
                } else null
            }
        }

        fun notNullOf(ident: String): ClassicalType {
            return of(ident) ?: throw IllegalArgumentException("Unknown type: $ident")
        }
    }
}

class ClassicalStructType(val ident: String) : ClassicalType {
    override fun toString(): String {
        TODO("Not yet implemented")
    }

    override fun createVariable(ident: String, location: Location?): ClassicalVariable {
        TODO("Not yet implemented")
    }

    override fun equals(other: Any?): Boolean {
        return other is ClassicalStructType && ident == other.ident
    }

    override fun hashCode(): Int {
        return ident.hashCode()
    }
}

class ClassicalListType(val elementType: ClassicalType) : ClassicalType {
    override fun toString(): String {
        return "[$elementType]"
    }

    override fun createVariable(ident: String, location: Location?): ClassicalVariable {
        return ListVariable(ident, elementType, location)
    }

    override fun equals(other: Any?): Boolean {
        return other is ClassicalListType && elementType == other.elementType
    }

    override fun hashCode(): Int {
        return elementType.hashCode() * 131
    }
}

enum class ClassicalBasicType : ClassicalType {
    Bool {
        override fun createVariable(ident: String, location: Location?): ClassicalVariable {
            return BoolVariable(ident, location)
        }
    },

    Int {
        override fun createVariable(ident: String, location: Location?): ClassicalVariable {
            return IntVariable(ident, location)
        }
    },

    Float {
        override fun createVariable(ident: String, location: Location?): ClassicalVariable {
            TODO("Not yet implemented")
        }
    },

    Complex {
        override fun createVariable(ident: String, location: Location?): ClassicalVariable {
            TODO("Not yet implemented")
        }
    },

    Bits {
        override fun createVariable(ident: String, location: Location?): ClassicalVariable {
            return BitsVariable(ident, location)
        }
    },

    Mat {
        override fun createVariable(ident: String, location: Location?): ClassicalVariable {
            TODO("Not yet implemented")
        }
    };

    override fun toString() = this.name

    companion object {
        inline fun <reified T: Any> from(): ClassicalBasicType {
            return when (T::class) {
                Boolean::class -> Bool
                kotlin.Int::class -> Int
                kotlin.Float::class -> Float
                ComplexDouble::class -> Complex
                NDArray::class -> Mat
                else -> throw IllegalArgumentException("Unknown type: ${T::class}")
            }
        }

        fun <T: Any> from(type: KClass<T>): ClassicalBasicType {
            return when (type) {
                Boolean::class -> Bool
                kotlin.Int::class -> Int
                kotlin.Float::class -> Float
                ComplexDouble::class -> Complex
                NDArray::class -> Mat
                else -> throw IllegalArgumentException("Unknown type: $type")
            }
        }
    }
}

infix fun ClassicalExpr.matches(type: ClassicalType): Boolean {
    return this.type == type
}
