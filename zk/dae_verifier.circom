pragma circom 2.1.6;

include "../node_modules/circomlib/circuits/poseidon.circom";
include "../node_modules/circomlib/circuits/comparators.circom";

template DAEVerifier() {
    // --- Public Inputs ---
    signal input expectedTelemetryCommitment;
    signal input challengeNonce;
    signal input binaryDigest;
    signal input maxLatencyNs;
    signal input maxAllocBytes;

    // --- Private Inputs ---
    signal input measuredLatencyNs;
    signal input measuredAllocBytes;
    signal input measuredClockDriftTicks;
    signal input executionSalt;

    // --- 1. Freshness & Telemetry Commitment ---
    component telemetryHasher = Poseidon(5);
    telemetryHasher.inputs[0] <-- binaryDigest;
    telemetryHasher.inputs[1] <-- challengeNonce;
    telemetryHasher.inputs[2] <-- measuredLatencyNs;
    telemetryHasher.inputs[3] <-- measuredAllocBytes;
    telemetryHasher.inputs[4] <-- executionSalt;

    telemetryHasher.out === expectedTelemetryCommitment;

    // --- 2. Invariant Boundary Checks ---
    component allocCheck = LessEqThan(64);
    allocCheck.in[0] <-- measuredAllocBytes;
    allocCheck.in[1] <-- maxAllocBytes;
    allocCheck.out === 1;

    component latencyCheck = LessEqThan(64);
    latencyCheck.in[0] <-- measuredLatencyNs;
    latencyCheck.in[1] <-- maxLatencyNs;
    latencyCheck.out === 1;

    component driftCheck = LessEqThan(32);
    driftCheck.in[0] <-- measuredClockDriftTicks;
    driftCheck.in[1] <-- 50;
    driftCheck.out === 1;
}

component main {public [expectedTelemetryCommitment, challengeNonce, binaryDigest, maxLatencyNs, maxAllocBytes]} = DAEVerifier();
