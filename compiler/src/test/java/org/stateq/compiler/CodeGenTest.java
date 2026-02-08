package org.stateq.compiler;

import org.junit.jupiter.api.Test;
import org.stateq.compiler.language.CQivmCodeGenerator;

import static org.stateq.util.TestUtils.*;
import static org.stateq.util.TestUtilsKt.*;

public class CodeGenTest {

    private final CodeGenerator codegen = new CQivmCodeGenerator();

    private void compile(String source) {
        assertNoCompileErrors(source, code -> {
            var target = ModuleCompiler.INSTANCE.compileModule(code).dumpCode(codegen);
            System.out.println("================================");
            printCodeWithLineNumber(code);
            System.out.println("--------------------------------");
            printCodeWithLineNumber(target);
            System.out.println("================================");
        });
    }

    @Test
    void testSimpleOperationXYZMul() {
        this.compile("""
             operation XYZ($psi) {
                Z Y X $psi;
             }
        """);
    }

    @Test
    void testSimpleOperationXYZConcat() {
        this.compile("""
             operation XYZ($psi: 3) {
                X.Y.Z $psi;
             }
        """);
    }

    @Test
    void testSimpleOperationDagger() {
        this.compile("""
            operation Foo($psi) {
                X! Y $psi;
            }
        """);
    }

    @Test
    void testSimpleOperationTensorPower() {
        this.compile("""
             operation DoubleHadamard($psi: 2) {
                H@2 $psi;
             }
        """);
    }

    @Test
    void testSimpleOperationTensorPowerN() {
        this.compile("""
             operation NFoldHadamard[n: Int]($psi: n) {
                H@n $psi;
             }
        """);
    }

    @Test
    void testSimpleOperationCNot() {
        this.compile("""
             operation CX($psi: 2) {
                qif &psi[0] {
                    X $psi[1];
                }
             }
        """);
    }

    @Test
    void testSimpleOperationCCNot() {
        this.compile("""
             operation CCX($psi: 3) {
                qif &psi[0] and &psi[1] {
                    X $psi[2];
                }
             }
        """);
    }

    @Test
    void testSimpleOperationQifElse() {
        this.compile("""
             operation Foo($psi: 4) {
                qif (&psi[1] and &psi[2]) or !&psi[0] {
                    X Y $psi[3];
                } else {
                    Y Z $psi[3];
                }
             }
        """);
    }

    @Test
    void testSimpleOperationClassicalParam() {
        this.compile("""
             operation Foo[n: Int]($psi: n) {
                H $psi[0];
             }
        """);
    }

    @Test
    void testSimpleOperationAutoInferQvarSize() {
        this.compile("""
             operation Foo($psi: ?n) {
                H $psi[0];
             }
        """);
    }

    @Test
    void testSimpleOperationBitsCompre() {
        this.compile("""
             operation U[bits: Bits]($psi: ?n) {
                i <- bits | H $psi[i];
             }
        """);
    }

    @Test
    void testSimpleOperationForLoop() {
        this.compile("""
             operation U[bits: Bits]($psi: ?n) {
                for i in bits {
                    H $psi[i];
                }
             }
        """);
    }

    @Test
    void testSimpleOperationListCompre() {
        this.compile("""
             operation U[indexes: [Int]]($psi: ?n) {
                each i in indexes | H $psi[i];
             }
        """);
    }

    @Test
    void testCCNotCall() {
        this.compile("""
             operation CCX($psi: 3) {
                qif &psi[0] and &psi[1] {
                    X $psi[2];
                }
             }
             
             operation Oracle($psi: ?n) {
                 i in [0 : n-3] | CCX $psi[i : i+3];
             }
        """);
    }

    @Test
    void testSimpleProgram() {
        this.compile("""
            program Sample[] {
                measure H |0>;
            }
        """);
    }

    @Test
    void testProgramMeasurementSlice() {
        this.compile("""
            program Sample[] {
                measure[0:2] H@4 |0000>;
            }
        """);
    }

    @Test
    void testSimpleCtrlStatement() {
        this.compile("""
            operation Foo($psi: 2) {
                H $psi[0] ctrl &psi[1];
            }
        """);
    }

    @Test
    void testSimpleBernsteinVaziraniAlgorithm() {
        this.compile("""
            operation BvOracle[bits: Int] ($psi: ?n) {
                for i in [0 : n - 1] {
                    if (1 << i) & bits != 0 {
                        X $psi[n - 1] ctrl &psi[i];
                    }
                }
            }
        
            program BernsteinVazirani[bits: Int, n: Int] shot(128) {
                measure[:n] H@n.I BvOracle[bits] H@n.(Z H) |0'n+1⟩;
            }
        """);
    }

    @Test
    void testQFT() {
        this.compile("""
            operation QFT($phi: ?n) {
                for i in [0 : n] {
                    H $phi[n - 1 - i];
                    for j in [i + 1 : n] {
                        P[pi / 2**(j-i)] $phi[n - 1 - i] ctrl &phi[n - 1 - j];
                    }
                }
                each i in [0 : n / 2] | SWP $phi[i, n - i - 1];
            }
        """);
    }

    @Test
    void testWithStatement() {
        this.compile("""
            operation Foo($psi: 2) {
                with X $psi[0] {
                    X $psi[1];
                }
            }
        """);
    }

    @Test
    void testPhaseAdder() {
        this.compile(""" 
            operation QFT($phi: ?n) {
                for i in [0 : n] {
                    H $phi[n - 1 - i];
                    for j in [i + 1 : n] {
                        P[pi / 2**(j-i)] $phi[n - 1 - i] ctrl &phi[n - 1 - j];
                    }
                }
                each i in [0 : n / 2] | SWP $phi[i, n - i - 1];
            }
            
            operation PhaseAdder[value: Int]($phi: ?n) {
                for i in [0 : n] {
                    if value & (1 << (n - 1 - i)) != 0 {
                        each j in [0 .. i] | P[pi * (2**(j-i))] $phi[j];
                    }
                }
            }
            
            operation Adder[value: Int]($phi: ?n) {
                with QFT $phi {
                    PhaseAdder[value] $phi;
                }
            }
        """);
    }

    @Test
    void testShorAlgorithm() {
        this.compile("""
            // 4n+2 qubits shor's algorithm
            
            extern func modInv(a: Int, m: Int) => Int;
            
            operation QFT($phi: ?n) {
                for i in [0 : n] {
                    H $phi[n - 1 - i];
                    for j in [i + 1 : n] {
                        P[pi / 2**(j-i)] $phi[n - 1 - i] ctrl &phi[n - 1 - j];
                    }
                }
                each i in [0 : n / 2] | SWP $phi[i, n - i - 1];
            }
            
            operation PhaseAdder[value: Int]($phi: ?n) {
                let p: Int = 2**n;
                for i in [0 : n] {
                    if ((value % p + p) % p)  & (1 << (n - 1 - i)) != 0 {
                        each j in [0 .. i] | P[pi / (2**(i-j))] $phi[j];
                    }
                }
            }
            
            operation Adder[value: Int]($phi: ?n) {
                with QFT $phi {
                    PhaseAdder[value] $phi;
                }
            }
            
            operation ModAdder[value: Int, modValue: Int]($phi: ?n) {
                let addition = (value % modValue + modValue) % modValue;
                $phi.|0⟩ as $work;
                |0⟩ as $ancilla;
                Adder[-modValue] Adder[addition] $work;
                X $ancilla ctrl &work[n];
                Adder[modValue] $work ctrl &ancilla;
                with Adder[-addition] $work {
                    with X $work[n] {
                        X $ancilla ctrl &work[n];
                    }
                }
            }
            
            operation ModAdderMult[value: Int, modValue: Int]($phi: ?m) {
                let mult: Int = (value % modValue + modValue) % modValue;
                let n: Int = m / 2;
                for i in [0 : n] {
                    let a = (2**i % modValue) * mult % modValue;
                    ModAdder[a, modValue] $phi[n : 2*n] ctrl &phi[i];
                }
            }
            
            operation ModMultiplier[value: Int, modValue: Int]($phi: ?n) {
                let mult: Int = (value % modValue + modValue) % modValue;
                let multInv: Int = modValue - modInv(mult, modValue);
                ModAdderMult[mult, modValue] $phi.|0'n⟩ as $work;
                each i in [0 : n] | SWP $work[i, n + i];
                ModAdderMult[multInv, modValue] $work;
            }
            
            program ShorPeriodFinder[a: Int, modValue: Int] shot(4096) {
                let n: Int = ceil(log2i(modValue));
                let m: Int = 2 * n;
                H@m |0'm⟩ as $psi;
                |0b1'n⟩ as $ancilla;
                for i in [: m] {
                    ModMultiplier[a**(2**i) % modValue, modValue] $ancilla ctrl &psi[i];
                }
                measure QFT! $psi;
            }
        """);
    }

}
