package org.stateq.polynomial

import org.junit.jupiter.api.Assertions
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class PolynomialTest {

    @ParameterizedTest
    @CsvSource(value = [
        "3, 5, 15",
        "0, 0, 0",
        "0, 114, 0",
        "514, 0, 0",
        "0, x^2, 0",
        "x, y, x*y",
        "x^2, y, x^2*y",
        "x^2, y^3*z, x^2*y^3*z",
        "x^2, x^3, x^5",
        "x^2, x*y^2, x^3*y^2",
        "-4*x, -7, 28*x",
        "3*x, 2*y, 6*x*y",
        "x*y, -z*w, -x*y*z*w",
        "-15*x^3, 8*x^2*y^7, -120*x^5*y^7",
        "50*x*y, -6*x^3*y^5, -300*x^4*y^6",
        "y^3, 0, 0",
        "3*x, y^4, 3*x*y^4",
        "-6*x*y^2, -5*x^3*y, 30*x^4*y^3",
        "8*x*y*z, -3*x, -24*x^2*y*z",
        "5*x*y^3, z^2, 5*x*y^3*z^2",
        "10*x, 12*x*y*z, 120*x^2*y*z",
        "-x, 6*x*y, -6*x^2*y",
        "11*x^2, 2*y*z^4, 22*x^2*y*z^4",
        "-9*x*y^11, -y, 9*x*y^12",
        "66*x^3*y, -x^5*z^2, -66*x^8*y*z^2",
        "7*x*z, 7*y^6, 49*x*y^6*z",
        "60*y*z, -6*x, -360*x*y*z",
        "90*x*y, -x, -90x^2*y",
        "11x^5, -11y, -121x^5y",
        "-x*z, -x*y, x^2*y*z",
        "x*y^3*z, -6*x*y, -6*x^2*y^4*z",
        "7*x*y, 19*x*y*z^5, 133*x^2*y^2*z^5",
        "-9*x*y*z, -9*x*y*z, 81*x^2*y^2*z^2",
        "12*x*y*z*m*n, m, 12*x*y*z*m^2*n"
    ])
    fun `test monomial multiply`(
        lhs: IntMonomial, rhs: IntMonomial, expected: IntMonomial?
    ) {
        val result = lhs * rhs
        println("$lhs * $rhs = $result")
        Assertions.assertEquals(expected, result)
    }

    @ParameterizedTest
    @CsvSource(value = [
        "114, 514, 628",
        "0, 114, 114",
        "514, 0, 514",
        "0, x^3, x^3",
        "y^2*x^3*z^4, 0, y^2*x^3*z^4",
        "2*x^2, 3*y^2, 2*x^2+3*y^2",
        "4*x^5, 7*x^5, 11*x^5",
        "-6*x*y^2, -3*x, -3*x+-6*x*y^2",
        "8*x*y*z+5, -5*x^3*y, 8*x*y*z+-5*x^3*y",
        "5*x*y^3, z^2, 5*x*y^3+z^2",
        "-y, -x, -x+-y",
        "12*x*y*z, -9*x*y*z, 3*x*y*z",
        "7*y^6, 2*y*z^4, 7*y^6+2*y*z^4",
        "6*x*y, 10*x*y, 16*x*y",
        "66*x^3*y, 60*z, 66*x^3*y+60*z",
        "7*x*z, 11*x^2, 7*x*z+11*x^2",
        "-x^5*z^2, -6*x, -x^5*z^2+-6*x",
        "11*x*y, 11*x*y, 22*x*y",
        "7*x*z, 7*y^6, 7*x*z+7*y^6",
        "6*y*z, -6*x, 6*y*z+-6*x",
        "9*x, -10*x, -x",
        "11*x^5, -11*x, 11*x^5+-11*x",
        "-x*z, -x*y, -x*z+-x*y",
        "x*y^3*z, 19*x*y*z^5, x*y^3*z+19*x*y*z^5",
        "7*x*y, -6*x*y, x*y",
        "-9*x*y*z, -9*x*y*z, -18*x*y*z",
        "x*y*z*m*n, 0, x*y*z*m*n"
    ])
    fun testMonomialAdd(lhs: IntMonomial, rhs: IntMonomial, expected: IntPolynomial?) {
        val result = lhs + rhs
        println("$lhs + $rhs = $result")
        Assertions.assertEquals(expected, result)
    }

    @ParameterizedTest
    @CsvSource(value = [
        "114, 514, 628",
        "0, 114, 114",
        "514, 0, 514",
        "0, x^3, x^3",
        "y^2*x^3*z^4, 0, y^2*x^3*z^4",
        "2*x^2, 3*y^2, 2*x^2+3*y^2",
        "4*x^5, 7*x^5, 11*x^5",
        "-6*x*y^2, -3*x, -3*x+-6*x*y^2",
        "8*x*y*z+5, -5*x^3*y, 8*x*y*z+-5*x^3*y+5",
        "5*x*y^3+5*z^2, z^2, 5*x*y^3+6*z^2",
        "x^5+-y, -x+y, x^5+-x",
        "12*x*y*z, -9*x*y*z, 3*x*y*z",
        "7*y^6+-2*y*z^4, 2*y*z^4+6*x, 7*y^6+6*x",
        "6*x*y+-x^7, 10*x*y+y^8, 16*x*y+-x^7+y^8",
        "66*x^3*y+-9, 60*z+8, 66*x^3*y+60*z+-1",
        "7*x*z+x^2, 11*x^2+y+z, 7*x*z+12*x^2+y+z",
        "-x^5*z^2+-x, -6*x, -x^5*z^2+-7*x",
        "11*x*y+10*x, 11*x*y+x, 22*x*y+11*x",
        "7*x*z+y^6, 7*y^6, 7*x*z+8*y^6",
        "6*y*z-x*y, -6*x, 6*y*z+-6*x-x*y",
        "9*x-1, -10*x, -x-1",
        "11*x^5-y, -11*x, 11*x^5+-11*x-y",
        "-x*z+-x*y, -x*y, -x*z+-2*x*y",
        "x*y^3*z, 19*x*y*z^5, x*y^3*z+19*x*y*z^5",
        "7*x*y, -6*x*y, x*y",
        "-9*x*y*z, -9*x*y*z, -18*x*y*z",
        "x*y*z*m*n, 0, x*y*z*m*n"
    ])
    fun testPolynomialAdd(lhs: IntPolynomial, rhs: IntPolynomial, expected: IntPolynomial?) {
        val result = lhs + rhs
        println("$lhs + $rhs = $result")
        Assertions.assertEquals(expected, result)
    }

    @ParameterizedTest
    @CsvSource(value = [
        "114, 514, 58596",
        "0, 114, 0",
        "514, 0, 0",
        "0, x^3, 0",
        "y^2*x^3*z^4, 1, y^2*x^3*z^4",
        "2*x^2, 3*y^2, 6*y^2*x^2",
        "4*x^5, 7*x^5, 28*x^10",
        "x+2, y+3, x*y + 3*x + 2*y + 6",
        "50*x*y, -6*x^3*y^5, -300*x^4*y^6",
        "y^3, 0, 0",
        "3*x+x, y^4, 3*x*y^4+x*y^4",
        "-6*x*y^2+2*x, -5*x^3*y, 30*x^4*y^3+-10*x^4*y",
        "8*x*y*z, -3*x+z, -24*x^2*y*z+8*x*y*z^2",
        "5*x*y^3, z^2+1, 5*x*y^3*z^2+5*x^1*y^3",
        "10*x-1, 12*x*y*z, 120*x^2*y*z-12*x*y*z",
        "-x+3, 6*x*y, -6*x^2*y+18*x*y",
        "11*x^2, 2*y*z^4+10*x, 22*x^2*y*z^4+110*x^3",
        "-9*x*y^11, -y, 9*x*y^12",
        "66*x^3*y+8*x*y, -x^5*z^2, -66*x^8*y*z^2+-8*x^6*y*z^2",
        "7*x*z+y, 7*y^6, 49*x*y^6*z+7*y^7",
        "60*y*z, -6*x+6*z, -360*x*y*z+360*y*z^2",
        "x*y, -x+y, -x^2*y+x*y^2",
        "x^5, -11*y+3, -11*x^5*y+3*x^5",
        "-x*z+y, -x*y, x^2*y*z+-x*y^2",
        "x*y^3*z, -6*x*y, -6*x^2*y^4*z",
        "7*x*y, 19*x*y*z^5, 133*x^2*y^2*z^5",
        "-9*x*y*z+m, -9*x*y*z+n, 81*x^2*y^2*z^2+-9*x*y*z*n+-9*x*y*z*m+n*m",
        "x*y*z*m*n+m, m, x*y*z*m^2*n+m^2"
    ])
    fun testPolynomialMul(lhs: IntPolynomial, rhs: IntPolynomial, expected: IntPolynomial?) {
        val result = lhs * rhs
        println("($lhs) * ($rhs) = $result")
        Assertions.assertEquals(expected, result)
    }
}
