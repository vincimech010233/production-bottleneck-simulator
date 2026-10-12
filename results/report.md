# Production Bottleneck Simulator

OBSERVED synthetic scenarios.

{&#x27;baseline&#x27;: {&#x27;throughput_mean&#x27;: 0.37222222222222223, &#x27;throughput_sample_sd&#x27;: 0.004811252243246896, &#x27;completed&#x27;: [45, 44, 45]}, &#x27;candidate&#x27;: {&#x27;throughput_mean&#x27;: 0.65, &#x27;throughput_sample_sd&#x27;: 0.0, &#x27;completed&#x27;: [78, 78, 78]}}

3 seeds, startup and censoring retained; jitter is integer uniform +/-5%, rounding to >=1 ms, LCG non-cryptographic. Draw order can change across scenarios; not paired common random numbers.
