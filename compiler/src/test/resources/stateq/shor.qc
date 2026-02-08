gate Rk[k: Float] = U[0, 2 * pi / (2 ** n), 0];

operation QFT($psi: ?n) {
    for i in [: n] {
        H $psi[i];
        [i + 1 : n] | Rk[_ - i + 1] $psi[j] ctrl &psi[i];
    }
    i <- [: n / 2] | SWP $psi[i, n - i - 1];
}

operation PhaseAdd[value: Int]($psi: ?n) {
    for i in [: n] {
        if (value % (2 ** n)) & (1 << n - 1 - i) != 0 {
            j <- [0 .. i] | P[pi * 2**(j-i)] $psi[j];
        }
    }
}

operation Add[value: Int]($psi: ?n) {
    QFT! PhaseAdd[value] QFT $psi;
}

operation ModReduce[value: Int, order: Int]($psi: ?n) {
    Add[-value * (2**order)] $psi.$carry as $psiWithCarry;
    Add[value * (2**order)] $psi.|1⟩ ctrl &psiWithCarry[-1];
}

operation Mod[value: Int]($psi: ?n) {
    [: n] | ModReduce[value, _] $psi;
}

operation ModMul[value: Int, mod: Int]($base: ?n) {
    |0'n⟩ as $result;
    for i in [: n] {
        Mod[mod] Add[value * (2**i)] $result ctrl &base[i];
    }
    Swap($result, $base);
}

program ShorPeriodFinder[a: Int, n: Int] {
    H@? |0'2*n⟩ as $psi;
    [: 2*n] | ModMul[2**_] |1'n⟩ ctrl &psi;
    measure QFT! $psi;
}
