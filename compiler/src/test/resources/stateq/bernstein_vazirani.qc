operation BvOracle[bits: Int, n: Int] ($psi: n + 1) {
    for i in [0 : n] {
        if (1 << i) & bits != 0 {
            X $psi[n] ctrl &psi[i];
        }
    }
}

program BernsteinVazirani[bits: Int, n: Int] {
    measure[:n-1] H@n.I BvOracle[bits, n] H@n.(Z H) |0'n+1⟩;
}
