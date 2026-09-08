# ΔΔG iterations — Raspberry Pi and EC2

Times are whole-process seconds unless the scope column says kernel_compute; transcript runs use the actual OligoAI v2 oracle, and site runs use direct RNAduplex. Synthetic estimate-sites workloads are throughput calibrations, not genome discovery benchmarks. Transcript mode uses four RNAplex workers plus one concurrent RNAduplex process unless stated; the worker column gives each site/kernel run’s worker count. Speedups are reported only for iterations that passed every output-field comparison.

| Iteration | Workers | Scope | Status | Baseline | Baseline median s | oofft median s | Speedup |
| --- | ---: | --- | --- | --- | ---: | ---: | ---: |
| [20260907T235249.316395-initial-fixtures](iterations/20260907T235249.316395-initial-fixtures/result.json) | 4 | whole_process | FAILED | oligoai | 0.085255 | — | — |
| [20260907T235400.837653-corrected-energy-parser](iterations/20260907T235400.837653-corrected-energy-parser/result.json) | 4 | whole_process | PASS | oligoai | 0.057567 | 0.016271 | 3.54× |
| [20260907T235448.555483-real-unique128](iterations/20260907T235448.555483-real-unique128/result.json) | 4 | whole_process | PASS | oligoai | 19.872153 | 5.094576 | 3.90× |
| [20260907T235651.916384-real-repeated128](iterations/20260907T235651.916384-real-repeated128/result.json) | 4 | whole_process | PASS | oligoai | 19.563825 | 0.657610 | 29.75× |
| [20260907T235752.704112-real-exact1000](iterations/20260907T235752.704112-real-exact1000/result.json) | 4 | whole_process | PASS | oligoai | 0.098735 | 0.010769 | 9.17× |
| [20260908-direct-interval-full-chunk-validation](iterations/20260908-direct-interval-full-chunk-validation/result.json) | 1 | interval_read_validation | PASS | — | — | 28.294506 | — |
| [20260908-full-scn2a-forward-chunked](iterations/20260908-full-scn2a-forward-chunked/result.json) | 8 | archive_worker, ddg_worker, discovery_worker | INTERRUPTED | — | — | 82.053446 | — |
| [20260908-full-scn2a-paired-direct](iterations/20260908-full-scn2a-paired-direct/result.json) | 8 | archive_worker, ddg_worker, discovery_worker | INTERRUPTED | — | — | 42.293942 | — |
| [20260908-full-scn2a-unmasked-report-ddg](iterations/20260908-full-scn2a-unmasked-report-ddg/result.json) | 8 | whole_process | FAILED | — | — | — | — |
| [20260908T000443.743071-final-indels-16-workers1](iterations/20260908T000443.743071-final-indels-16-workers1/result.json) | 1 | whole_process | PASS | oligoai | 0.070867 | 0.016786 | 4.22× |
| [20260908T000443.931377-final-indels-16-workers4](iterations/20260908T000443.931377-final-indels-16-workers4/result.json) | 4 | whole_process | PASS | oligoai | 0.048777 | 0.014285 | 3.41× |
| [20260908T000444.081733-final-indels-18-workers1](iterations/20260908T000444.081733-final-indels-18-workers1/result.json) | 1 | whole_process | PASS | oligoai | 0.053850 | 0.023092 | 2.33× |
| [20260908T000444.245957-final-indels-18-workers4](iterations/20260908T000444.245957-final-indels-18-workers4/result.json) | 4 | whole_process | PASS | oligoai | 0.053203 | 0.015197 | 3.50× |
| [20260908T000444.399723-final-indels-20-workers1](iterations/20260908T000444.399723-final-indels-20-workers1/result.json) | 1 | whole_process | PASS | oligoai | 0.061737 | 0.028530 | 2.16× |
| [20260908T000444.576111-final-indels-20-workers4](iterations/20260908T000444.576111-final-indels-20-workers4/result.json) | 4 | whole_process | PASS | oligoai | 0.058839 | 0.021310 | 2.76× |
| [20260908T090630.514885-site-annotation-oracle](iterations/20260908T090630.514885-site-annotation-oracle/result.json) | 4 | whole_process | PASS | viennarna | 0.005795 | 0.017169 | 0.34× |
| [20260908T090630.654781-renamed-whole-transcript-regression](iterations/20260908T090630.654781-renamed-whole-transcript-regression/result.json) | 4 | whole_process | PASS | oligoai | 0.070809 | 0.015809 | 4.48× |
| [20260908T090908.866818-site-annotation-oracle](iterations/20260908T090908.866818-site-annotation-oracle/result.json) | 4 | whole_process | PASS | viennarna | 0.010070 | 0.028074 | 0.36× |
| [20260908T090908.986264-final-whole-transcript-after-site-integration](iterations/20260908T090908.986264-final-whole-transcript-after-site-integration/result.json) | 4 | whole_process | PASS | oligoai | 0.055214 | 0.017177 | 3.21× |
| [20260908T091058.575096-site-annotation-oracle](iterations/20260908T091058.575096-site-annotation-oracle/result.json) | 4 | whole_process | PASS | viennarna | 0.008308 | 0.024080 | 0.35× |
| [20260908T091757.493800-estimate-sites-10000-records1](iterations/20260908T091757.493800-estimate-sites-10000-records1/result.json) | 4 | whole_process | PASS | viennarna | 2.191423 | 0.830383 | 2.64× |
| [20260908T091801.032967-estimate-sites-10000-records1000](iterations/20260908T091801.032967-estimate-sites-10000-records1000/result.json) | 4 | whole_process | PASS | viennarna | 2.187430 | 5.554482 | 0.39× |
| [20260908T093245.077318-estimate-sites-10000-records1000-rust](iterations/20260908T093245.077318-estimate-sites-10000-records1000-rust/result.json) | 4 | whole_process | PASS | viennarna | 2.188773 | 0.588724 | 3.72× |
| [20260908T093724.245391-estimate-sites-10000-records1000-rust](iterations/20260908T093724.245391-estimate-sites-10000-records1000-rust/result.json) | 4 | whole_process | PASS | viennarna | 2.191184 | 0.342363 | 6.40× |
| [20260908T093727.248569-estimate-sites-10000-records1-rust](iterations/20260908T093727.248569-estimate-sites-10000-records1-rust/result.json) | 4 | whole_process | PASS | viennarna | 2.203220 | 0.271380 | 8.12× |
| [20260908T094541.144928-scalar-kernel-golden-cpu1](iterations/20260908T094541.144928-scalar-kernel-golden-cpu1/result.json) | 1 | kernel_compute | PASS | — | — | 1.415999 | — |
| [20260908T094809.605757-factored-kernel-golden-cpu1](iterations/20260908T094809.605757-factored-kernel-golden-cpu1/result.json) | 1 | kernel_compute | PASS | — | — | 1.191490 | — |
| [20260908T094810.982655-estimate-sites-10000-records1000-rust](iterations/20260908T094810.982655-estimate-sites-10000-records1000-rust/result.json) | 4 | whole_process | PASS | viennarna | 2.194705 | 0.347857 | 6.31× |
| [20260908T100036.410967-reusable-workspace-golden](iterations/20260908T100036.410967-reusable-workspace-golden/result.json) | 1 | kernel_compute | PASS | native-baseline | 1.182122 | 1.177369 | 1.00× |
| [20260908T100044.310102-estimate-sites-10000-records1000-rust](iterations/20260908T100044.310102-estimate-sites-10000-records1000-rust/result.json) | 4 | whole_process | PASS | viennarna | 2.188926 | 0.347086 | 6.31× |
| [20260908T100508.278731-neon-loop-reduction-golden](iterations/20260908T100508.278731-neon-loop-reduction-golden/result.json) | 1 | kernel_compute | PASS | native-baseline | 1.177430 | 1.219661 | 0.97× |
| [20260908T100516.307891-estimate-sites-10000-records1000-rust](iterations/20260908T100516.307891-estimate-sites-10000-records1000-rust/result.json) | 4 | whole_process | PASS | viennarna | 2.285131 | 0.331631 | 6.89× |
| [20260908T100638.793666-real-sites-discovery-100](iterations/20260908T100638.793666-real-sites-discovery-100/result.json) | 1 | site_discovery | COMPLETE (not energy-verified) | — | — | — | — |
| [20260908T101024.131957-real-sites-discovery-1000](iterations/20260908T101024.131957-real-sites-discovery-1000/result.json) | 1 | site_discovery | COMPLETE (not energy-verified) | — | — | — | — |
| [20260908T101316.880403-real-site-annotation](iterations/20260908T101316.880403-real-site-annotation/result.json) | 4 | whole_process | PASS | vienna-baseline | 0.283788 | 0.110387 | 2.57× |
| [20260908T101658.719697-estimate-sites-10000-records1000-rust](iterations/20260908T101658.719697-estimate-sites-10000-records1000-rust/result.json) | 4 | whole_process | PASS | viennarna | 2.190570 | 0.323176 | 6.78× |
| [20260908T101701.697684-real-site-annotation](iterations/20260908T101701.697684-real-site-annotation/result.json) | 4 | whole_process | PASS | vienna-baseline | 0.268964 | 0.116273 | 2.31× |
| [20260908T102429.623503-real-site-annotation](iterations/20260908T102429.623503-real-site-annotation/result.json) | 4 | whole_process | PASS | vienna-baseline | 0.268976 | 0.098449 | 2.73× |
| [20260908T102431.200096-estimate-sites-10000-records1000-rust](iterations/20260908T102431.200096-estimate-sites-10000-records1000-rust/result.json) | 4 | whole_process | PASS | viennarna | 2.216232 | 0.203217 | 10.91× |
| [20260908T102704.663301-unrolled-small-loops-golden](iterations/20260908T102704.663301-unrolled-small-loops-golden/result.json) | 1 | kernel_compute | PASS | native-baseline | 1.180117 | 1.165652 | 1.01× |
| [20260908T102712.497049-real-site-annotation](iterations/20260908T102712.497049-real-site-annotation/result.json) | 4 | whole_process | PASS | vienna-baseline | 0.265044 | 0.099293 | 2.67× |
| [20260908T102914.989662-relaxed-matching-prune-golden](iterations/20260908T102914.989662-relaxed-matching-prune-golden/result.json) | 1 | kernel_compute | PASS | native-baseline | 1.145696 | 1.384314 | 0.83× |
| [20260908T102923.456151-real-site-annotation](iterations/20260908T102923.456151-real-site-annotation/result.json) | 4 | whole_process | PASS | vienna-baseline | 0.256157 | 0.103148 | 2.48× |
| [20260908T102924.993802-estimate-sites-10000-records1000-rust](iterations/20260908T102924.993802-estimate-sites-10000-records1000-rust/result.json) | 4 | whole_process | PASS | viennarna | 2.199724 | 0.236777 | 9.29× |
| [20260908T103259.402302-real-site-annotation](iterations/20260908T103259.402302-real-site-annotation/result.json) | 4 | whole_process | PASS | rust-baseline | 0.099773 | 0.097753 | 1.02× |
| [20260908T103301.806682-real-site-annotation](iterations/20260908T103301.806682-real-site-annotation/result.json) | 4 | whole_process | PASS | vienna-baseline | 0.258011 | 0.102191 | 2.52× |
| [20260908T103303.348632-estimate-sites-10000-records1000-rust](iterations/20260908T103303.348632-estimate-sites-10000-records1000-rust/result.json) | 4 | whole_process | PASS | viennarna | 2.191607 | 0.200250 | 10.94× |
| [20260908T103345.681632-real-sites-discovery-10000](iterations/20260908T103345.681632-real-sites-discovery-10000/result.json) | 1 | site_discovery | COMPLETE (not energy-verified) | — | — | — | — |
| [20260908T103353.260461-real-sites-discovery-100000](iterations/20260908T103353.260461-real-sites-discovery-100000/result.json) | 1 | site_discovery | COMPLETE (not energy-verified) | — | — | — | — |
| [20260908T103534.925438-real-site-annotation](iterations/20260908T103534.925438-real-site-annotation/result.json) | 4 | whole_process | PASS | vienna-baseline | 4.200932 | 1.290205 | 3.26× |
| [20260908T103554.623075-real-site-annotation](iterations/20260908T103554.623075-real-site-annotation/result.json) | 4 | whole_process | PASS | vienna-baseline | 38.922903 | 12.744874 | 3.05× |
| [20260908T104143.986110-real-site-annotation](iterations/20260908T104143.986110-real-site-annotation/result.json) | 4 | whole_process | PASS | rust-baseline | 12.454225 | 11.930965 | 1.04× |
| [20260908T104527.038836-real-site-annotation](iterations/20260908T104527.038836-real-site-annotation/result.json) | 4 | whole_process | PASS | rust-baseline | 11.999032 | 7.128468 | 1.68× |
| [20260908T104655.104800-real-site-annotation](iterations/20260908T104655.104800-real-site-annotation/result.json) | 4 | whole_process | PASS | vienna-baseline | 38.842693 | 7.180837 | 5.41× |
| [20260908T105047.125660-real-site-annotation](iterations/20260908T105047.125660-real-site-annotation/result.json) | 4 | whole_process | PASS | vienna-baseline | 4.181658 | 0.728725 | 5.74× |
| [20260908T105105.109056-estimate-sites-10000-records1000-rust](iterations/20260908T105105.109056-estimate-sites-10000-records1000-rust/result.json) | 4 | whole_process | PASS | viennarna | 2.192051 | 0.201977 | 10.85× |
| [20260908T105315.328753-real-site-annotation](iterations/20260908T105315.328753-real-site-annotation/result.json) | 4 | whole_process | PASS | rust-baseline | 7.203925 | 6.352475 | 1.13× |
| [20260908T105426.272867-real-site-annotation](iterations/20260908T105426.272867-real-site-annotation/result.json) | 4 | whole_process | PASS | vienna-baseline | 28.108133 | 6.322299 | 4.45× |
| [20260908T110130.176140-ec2-initial-workers-1](iterations/20260908T110130.176140-ec2-initial-workers-1/result.json) | 1 | kernel_compute | PASS | — | — | 6.693740 | — |
| [20260908T110150.512281-ec2-initial-workers-2](iterations/20260908T110150.512281-ec2-initial-workers-2/result.json) | 2 | kernel_compute | PASS | — | — | 3.558603 | — |
| [20260908T110201.448043-ec2-initial-workers-4](iterations/20260908T110201.448043-ec2-initial-workers-4/result.json) | 4 | kernel_compute | PASS | — | — | 1.831031 | — |
| [20260908T110207.215066-ec2-initial-workers-8](iterations/20260908T110207.215066-ec2-initial-workers-8/result.json) | 8 | kernel_compute | PASS | — | — | 1.471562 | — |
| [20260908T111931.612056-ec2-avx2-real-vs-scalar](iterations/20260908T111931.612056-ec2-avx2-real-vs-scalar/result.json) | 1 | kernel_compute | PASS | native-baseline | 1.751660 | 2.720624 | 0.64× |
| [20260908T112312.660670-ec2-avx2-gather-real-vs-scalar](iterations/20260908T112312.660670-ec2-avx2-gather-real-vs-scalar/result.json) | 1 | kernel_compute | PASS | native-baseline | 1.776482 | 0.433502 | 4.10× |
| [20260908T112415.853057-ec2-avx2-vienna-library-workers-1](iterations/20260908T112415.853057-ec2-avx2-vienna-library-workers-1/result.json) | 1 | kernel_compute | PASS | vienna-library | 8.433567 | 0.445855 | 18.92× |
| [20260908T112453.319550-ec2-avx2-vienna-library-workers-2](iterations/20260908T112453.319550-ec2-avx2-vienna-library-workers-2/result.json) | 2 | kernel_compute | PASS | vienna-library | 4.588350 | 0.221900 | 20.68× |
| [20260908T112514.700132-ec2-avx2-vienna-library-workers-4](iterations/20260908T112514.700132-ec2-avx2-vienna-library-workers-4/result.json) | 4 | kernel_compute | PASS | vienna-library | 2.303818 | 0.116696 | 19.74× |
| [20260908T112526.603468-ec2-avx2-vienna-library-workers-8](iterations/20260908T112526.603468-ec2-avx2-vienna-library-workers-8/result.json) | 8 | kernel_compute | PASS | vienna-library | 1.992326 | 0.098444 | 20.24× |
| [20260908T113801.828561-ec2-auto-avx512-vs-avx2-workers-1](iterations/20260908T113801.828561-ec2-auto-avx512-vs-avx2-workers-1/result.json) | 1 | kernel_compute | PASS | native-baseline | 1.435789 | 1.015974 | 1.41× |
| [20260908T113813.580392-ec2-auto-avx512-vs-avx2-workers-8](iterations/20260908T113813.580392-ec2-auto-avx512-vs-avx2-workers-8/result.json) | 8 | kernel_compute | PASS | native-baseline | 0.322648 | 0.230016 | 1.40× |
| [20260908T114335.266528-ec2-prefix-unique-vs-scalar](iterations/20260908T114335.266528-ec2-prefix-unique-vs-scalar/result.json) | 1 | kernel_compute | PASS | native-baseline | 1.310785 | 0.711001 | 1.84× |
| [20260908T114929.744186-ec2-shared-avx512-unique](iterations/20260908T114929.744186-ec2-shared-avx512-unique/result.json) | 1 | kernel_compute | PASS | native-baseline | 0.803583 | 0.832403 | 0.97× |
| [20260908T115815.908861-ec2-minplus-avx512-unique](iterations/20260908T115815.908861-ec2-minplus-avx512-unique/result.json) | 1 | kernel_compute | PASS | native-baseline | 0.817811 | 1.245629 | 0.66× |
| [20260908T115853.730209-ec2-auto-avx512-vienna-library-workers-8](iterations/20260908T115853.730209-ec2-auto-avx512-vienna-library-workers-8/result.json) | 8 | kernel_compute | PASS | vienna-library | 2.020381 | 0.071810 | 28.14× |
| [20260908T120440.575550-real-site-annotation](iterations/20260908T120440.575550-real-site-annotation/result.json) | 8 | whole_process | PASS | vienna-baseline | 12.075393 | 2.251184 | 5.36× |
| [20260908T121058.529664-real-site-annotation](iterations/20260908T121058.529664-real-site-annotation/result.json) | 8 | whole_process | PASS | vienna-baseline | 10.052384 | 1.316229 | 7.64× |
| [20260908T121257.055425-real-site-annotation](iterations/20260908T121257.055425-real-site-annotation/result.json) | 8 | whole_process | PASS | vienna-baseline | 10.047490 | 1.155116 | 8.70× |
| [20260908T121453.889560-real-site-annotation](iterations/20260908T121453.889560-real-site-annotation/result.json) | 8 | whole_process | PASS | rust-baseline | 1.251042 | 1.185245 | 1.06× |
| [20260908T121721.230384-real-site-annotation](iterations/20260908T121721.230384-real-site-annotation/result.json) | 8 | whole_process | PASS | vienna-baseline | 10.020493 | 1.118794 | 8.96× |
| [20260908T122005.749442-real-site-annotation](iterations/20260908T122005.749442-real-site-annotation/result.json) | 8 | whole_process | PASS | vienna-baseline | 9.818999 | 1.016481 | 9.66× |
| [20260908T122228.749370-real-site-annotation](iterations/20260908T122228.749370-real-site-annotation/result.json) | 8 | whole_process | PASS | vienna-baseline | 9.758840 | 1.001300 | 9.75× |
| [20260908T122341.403813-real-site-annotation](iterations/20260908T122341.403813-real-site-annotation/result.json) | 4 | whole_process | PASS | vienna-baseline | 11.809776 | 1.106856 | 10.67× |
| [20260908T122554.400775-real-site-annotation](iterations/20260908T122554.400775-real-site-annotation/result.json) | 8 | whole_process | PASS | vienna-baseline | 9.784350 | 1.020828 | 9.58× |
| [20260908T122707.290092-real-site-annotation](iterations/20260908T122707.290092-real-site-annotation/result.json) | 4 | whole_process | PASS | vienna-baseline | 11.794865 | 1.116413 | 10.56× |
| [20260908T1300-full-reference-sequence-reuse](iterations/20260908T1300-full-reference-sequence-reuse/result.json) | 1 | sequence_reuse_census | COMPLETE (not energy-verified) | — | — | — | — |
| [20260908T130610.268465-ec2-shared-paths-full-reference-unique100k](iterations/20260908T130610.268465-ec2-shared-paths-full-reference-unique100k/result.json) | 1 | kernel_compute | PASS | native-baseline | 1.089606 | 1.301823 | 0.84× |
| [20260908T131502.058254-ec2-shared-paths-full-reference-all-unique](iterations/20260908T131502.058254-ec2-shared-paths-full-reference-all-unique/result.json) | 8 | kernel_compute | PASS | native-baseline | 1.925309 | 2.613517 | 0.74× |
| [20260908T131623.311715-ec2-shared-adjacent-full-reference-all-unique](iterations/20260908T131623.311715-ec2-shared-adjacent-full-reference-all-unique/result.json) | 8 | kernel_compute | PASS | native-baseline | 1.925656 | 1.926264 | 1.00× |
| [20260908T132022.190737-ec2-avx512-vienna-full-reference-all-unique](iterations/20260908T132022.190737-ec2-avx512-vienna-full-reference-all-unique/result.json) | 8 | kernel_compute | PASS | vienna-library | 63.975903 | 1.923879 | 33.25× |
| [20260908T154414.515307-pulp-neon-golden](iterations/20260908T154414.515307-pulp-neon-golden/result.json) | 1 | kernel_compute | PASS | native-baseline | 1.151240 | 1.004052 | 1.15× |
| [20260908T154421.822452-pulp-neon-real-unique](iterations/20260908T154421.822452-pulp-neon-real-unique/result.json) | 1 | kernel_compute | PASS | native-baseline | 2.643162 | 1.244699 | 2.12× |

The first fixture attempt failed because the parser read a structure parenthesis as the energy delimiter. Its raw output is retained. The corrected parser reads the final parenthesized energy.

Real workloads: 128 distinct non-exact SCN2A designs; 128 rows repeating 16 designs; and 1,000 exact SCN1A matches (the application shortcut). These are distinct performance scenarios, not interchangeable speedup claims.
