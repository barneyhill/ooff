# Classic-aligner benchmark iterations

Every invocation is retained, including preparation, failed attempts, and pilots.
Raw commands, logs, timing, and outputs are in each iteration directory.

The README uses powers-of-ten batches. Other sizes and the user-cancelled 26,643-ASO verification remain retained; see [batch-selection.json](classic/batch-selection.json).

| ID | Task | Wall seconds | Exit | Timeout |
| --- | --- | ---: | ---: | --- |
| [20260907T203213.914606-install-classic-tools](classic/iterations/20260907T203213.914606-install-classic-tools/result.json) | install-classic-tools | 5.476476 | 0 | False |
| [20260907T203219.438974-native-parallel-tests](classic/iterations/20260907T203219.438974-native-parallel-tests/result.json) | native-parallel-tests | 3.972421 | 0 | False |
| [20260907T203223.443156-native-parallel-build](classic/iterations/20260907T203223.443156-native-parallel-build/result.json) | native-parallel-build | 5.325040 | 0 | False |
| [20260907T203228.800837-prepare-eligible-reference](classic/iterations/20260907T203228.800837-prepare-eligible-reference/result.json) | prepare-eligible-reference | 34.444477 | 0 | False |
| [20260907T203303.291193-build-bwa-index](classic/iterations/20260907T203303.291193-build-bwa-index/result.json) | build-bwa-index | 2753.438990 | 0 | False |
| [20260907T212853.783160-bwa-fetch-inputs](classic/iterations/20260907T212853.783160-bwa-fetch-inputs/result.json) | bwa-fetch-inputs | 34.049889 | 0 | False |
| [20260907T212857.126376-minimap2-fetch-inputs](classic/iterations/20260907T212857.126376-minimap2-fetch-inputs/result.json) | minimap2-fetch-inputs | 5.226833 | 0 | False |
| [20260907T212857.531479-blast-fetch-inputs](classic/iterations/20260907T212857.531479-blast-fetch-inputs/result.json) | blast-fetch-inputs | 5.226681 | 0 | False |
| [20260907T212902.416037-build-minimap2-index](classic/iterations/20260907T212902.416037-build-minimap2-index/result.json) | build-minimap2-index | 30.786225 | 137 | False |
| [20260907T212902.815719-build-blast-index](classic/iterations/20260907T212902.815719-build-blast-index/result.json) | build-blast-index | 16.003740 | 0 | False |
| [20260907T212918.963009-pilot-blast-100-r1-search](classic/iterations/20260907T212918.963009-pilot-blast-100-r1-search/result.json) | pilot-blast-100-r1-search | 141.035213 | 0 | False |
| [20260907T212928.150841-pilot-bwa-100-r1-aln](classic/iterations/20260907T212928.150841-pilot-bwa-100-r1-aln/result.json) | pilot-bwa-100-r1-aln | 1.166783 | 0 | False |
| [20260907T212929.343196-pilot-bwa-100-r1-samse](classic/iterations/20260907T212929.343196-pilot-bwa-100-r1-samse/result.json) | pilot-bwa-100-r1-samse | 2.019317 | 0 | False |
| [20260907T212931.387286-pilot-bwa-100-r1-verify](classic/iterations/20260907T212931.387286-pilot-bwa-100-r1-verify/result.json) | pilot-bwa-100-r1-verify | 0.415027 | 0 | False |
| [20260907T213140.001308-pilot-blast-100-r1-verify](classic/iterations/20260907T213140.001308-pilot-blast-100-r1-verify/result.json) | pilot-blast-100-r1-verify | 0.414700 | 1 | False |
| [20260907T213444.987589-pilot-blast-identifier-repair](classic/iterations/20260907T213444.987589-pilot-blast-identifier-repair/result.json) | pilot-blast-identifier-repair | 0.515051 | 0 | False |
| [20260907T213451.582230-measurement-bwa-100-r1-aln](classic/iterations/20260907T213451.582230-measurement-bwa-100-r1-aln/result.json) | measurement-bwa-100-r1-aln | 1.065932 | 0 | False |
| [20260907T213452.651649-measurement-bwa-100-r1-samse](classic/iterations/20260907T213452.651649-measurement-bwa-100-r1-samse/result.json) | measurement-bwa-100-r1-samse | 2.018261 | 0 | False |
| [20260907T213454.671827-measurement-bwa-100-r1-verify](classic/iterations/20260907T213454.671827-measurement-bwa-100-r1-verify/result.json) | measurement-bwa-100-r1-verify | 0.465093 | 0 | False |
| [20260907T213455.146685-measurement-bwa-100-r2-aln](classic/iterations/20260907T213455.146685-measurement-bwa-100-r2-aln/result.json) | measurement-bwa-100-r2-aln | 1.015957 | 0 | False |
| [20260907T213456.164759-measurement-bwa-100-r2-samse](classic/iterations/20260907T213456.164759-measurement-bwa-100-r2-samse/result.json) | measurement-bwa-100-r2-samse | 1.867918 | 0 | False |
| [20260907T213458.034841-measurement-bwa-100-r2-verify](classic/iterations/20260907T213458.034841-measurement-bwa-100-r2-verify/result.json) | measurement-bwa-100-r2-verify | 0.414922 | 0 | False |
| [20260907T213458.459763-measurement-bwa-100-r3-aln](classic/iterations/20260907T213458.459763-measurement-bwa-100-r3-aln/result.json) | measurement-bwa-100-r3-aln | 1.016484 | 0 | False |
| [20260907T213459.478654-measurement-bwa-100-r3-samse](classic/iterations/20260907T213459.478654-measurement-bwa-100-r3-samse/result.json) | measurement-bwa-100-r3-samse | 1.867743 | 0 | False |
| [20260907T213501.348823-measurement-bwa-100-r3-verify](classic/iterations/20260907T213501.348823-measurement-bwa-100-r3-verify/result.json) | measurement-bwa-100-r3-verify | 0.414939 | 0 | False |
| [20260907T213501.774057-measurement-bwa-1000-r1-aln](classic/iterations/20260907T213501.774057-measurement-bwa-1000-r1-aln/result.json) | measurement-bwa-1000-r1-aln | 1.367644 | 0 | False |
| [20260907T213503.144299-measurement-bwa-1000-r1-samse](classic/iterations/20260907T213503.144299-measurement-bwa-1000-r1-samse/result.json) | measurement-bwa-1000-r1-samse | 2.319718 | 0 | False |
| [20260907T213505.466723-measurement-bwa-1000-r1-verify](classic/iterations/20260907T213505.466723-measurement-bwa-1000-r1-verify/result.json) | measurement-bwa-1000-r1-verify | 0.615312 | 0 | False |
| [20260907T213506.096168-measurement-bwa-1000-r2-aln](classic/iterations/20260907T213506.096168-measurement-bwa-1000-r2-aln/result.json) | measurement-bwa-1000-r2-aln | 1.317510 | 0 | False |
| [20260907T213507.417789-measurement-bwa-1000-r2-samse](classic/iterations/20260907T213507.417789-measurement-bwa-1000-r2-samse/result.json) | measurement-bwa-1000-r2-samse | 2.218911 | 0 | False |
| [20260907T213509.640171-measurement-bwa-1000-r2-verify](classic/iterations/20260907T213509.640171-measurement-bwa-1000-r2-verify/result.json) | measurement-bwa-1000-r2-verify | 0.615324 | 0 | False |
| [20260907T213510.271220-measurement-bwa-1000-r3-aln](classic/iterations/20260907T213510.271220-measurement-bwa-1000-r3-aln/result.json) | measurement-bwa-1000-r3-aln | 1.317601 | 0 | False |
| [20260907T213511.593081-measurement-bwa-1000-r3-samse](classic/iterations/20260907T213511.593081-measurement-bwa-1000-r3-samse/result.json) | measurement-bwa-1000-r3-samse | 2.219138 | 0 | False |
| [20260907T213513.816599-measurement-bwa-1000-r3-verify](classic/iterations/20260907T213513.816599-measurement-bwa-1000-r3-verify/result.json) | measurement-bwa-1000-r3-verify | 0.615780 | 0 | False |
| [20260907T213514.449338-measurement-bwa-5000-r1-aln](classic/iterations/20260907T213514.449338-measurement-bwa-5000-r1-aln/result.json) | measurement-bwa-5000-r1-aln | 8.686835 | 0 | False |
| [20260907T213523.141685-measurement-bwa-5000-r1-samse](classic/iterations/20260907T213523.141685-measurement-bwa-5000-r1-samse/result.json) | measurement-bwa-5000-r1-samse | 5.175112 | 0 | False |
| [20260907T213528.322187-measurement-bwa-5000-r1-verify](classic/iterations/20260907T213528.322187-measurement-bwa-5000-r1-verify/result.json) | measurement-bwa-5000-r1-verify | 1.868003 | 0 | False |
| [20260907T213530.226832-measurement-bwa-5000-r2-aln](classic/iterations/20260907T213530.226832-measurement-bwa-5000-r2-aln/result.json) | measurement-bwa-5000-r2-aln | 8.787930 | 0 | False |
| [20260907T213539.023222-measurement-bwa-5000-r2-samse](classic/iterations/20260907T213539.023222-measurement-bwa-5000-r2-samse/result.json) | measurement-bwa-5000-r2-samse | 5.222821 | 0 | False |
| [20260907T213544.254646-measurement-bwa-5000-r2-verify](classic/iterations/20260907T213544.254646-measurement-bwa-5000-r2-verify/result.json) | measurement-bwa-5000-r2-verify | 1.870154 | 0 | False |
| [20260907T213546.166190-measurement-bwa-5000-r3-aln](classic/iterations/20260907T213546.166190-measurement-bwa-5000-r3-aln/result.json) | measurement-bwa-5000-r3-aln | 8.637535 | 0 | False |
| [20260907T213554.816164-measurement-bwa-5000-r3-samse](classic/iterations/20260907T213554.816164-measurement-bwa-5000-r3-samse/result.json) | measurement-bwa-5000-r3-samse | 5.125968 | 0 | False |
| [20260907T213559.954221-measurement-bwa-5000-r3-verify](classic/iterations/20260907T213559.954221-measurement-bwa-5000-r3-verify/result.json) | measurement-bwa-5000-r3-verify | 1.868701 | 0 | False |
| [20260907T213601.870356-measurement-bwa-10000-r1-aln](classic/iterations/20260907T213601.870356-measurement-bwa-10000-r1-aln/result.json) | measurement-bwa-10000-r1-aln | 14.855955 | 0 | False |
| [20260907T213616.742033-measurement-bwa-10000-r1-samse](classic/iterations/20260907T213616.742033-measurement-bwa-10000-r1-samse/result.json) | measurement-bwa-10000-r1-samse | 8.733382 | 0 | False |
| [20260907T213625.491285-measurement-bwa-10000-r1-verify](classic/iterations/20260907T213625.491285-measurement-bwa-10000-r1-verify/result.json) | measurement-bwa-10000-r1-verify | 3.572102 | 0 | False |
| [20260907T213629.138889-measurement-bwa-10000-r2-aln](classic/iterations/20260907T213629.138889-measurement-bwa-10000-r2-aln/result.json) | measurement-bwa-10000-r2-aln | 14.605031 | 0 | False |
| [20260907T213643.765251-measurement-bwa-10000-r2-samse](classic/iterations/20260907T213643.765251-measurement-bwa-10000-r2-samse/result.json) | measurement-bwa-10000-r2-samse | 8.835069 | 0 | False |
| [20260907T213652.624809-measurement-bwa-10000-r2-verify](classic/iterations/20260907T213652.624809-measurement-bwa-10000-r2-verify/result.json) | measurement-bwa-10000-r2-verify | 3.520933 | 0 | False |
| [20260907T213656.230046-measurement-bwa-10000-r3-aln](classic/iterations/20260907T213656.230046-measurement-bwa-10000-r3-aln/result.json) | measurement-bwa-10000-r3-aln | 14.804631 | 0 | False |
| [20260907T213711.064118-measurement-bwa-10000-r3-samse](classic/iterations/20260907T213711.064118-measurement-bwa-10000-r3-samse/result.json) | measurement-bwa-10000-r3-samse | 8.783832 | 0 | False |
| [20260907T213719.875798-measurement-bwa-10000-r3-verify](classic/iterations/20260907T213719.875798-measurement-bwa-10000-r3-verify/result.json) | measurement-bwa-10000-r3-verify | 3.570852 | 0 | False |
| [20260907T213723.543965-measurement-bwa-26643-r1-aln](classic/iterations/20260907T213723.543965-measurement-bwa-26643-r1-aln/result.json) | measurement-bwa-26643-r1-aln | 41.127905 | 0 | False |
| [20260907T213804.708222-measurement-bwa-26643-r1-samse](classic/iterations/20260907T213804.708222-measurement-bwa-26643-r1-samse/result.json) | measurement-bwa-26643-r1-samse | 21.814179 | 0 | False |
| [20260907T213826.557780-measurement-bwa-26643-r1-verify](classic/iterations/20260907T213826.557780-measurement-bwa-26643-r1-verify/result.json) | measurement-bwa-26643-r1-verify | 9.385178 | 0 | False |
| [20260907T213836.125724-measurement-bwa-26643-r2-aln](classic/iterations/20260907T213836.125724-measurement-bwa-26643-r2-aln/result.json) | measurement-bwa-26643-r2-aln | 40.377179 | 0 | False |
| [20260907T213847.888138-measurement-blast-10-r1-search](classic/iterations/20260907T213847.888138-measurement-blast-10-r1-search/result.json) | measurement-blast-10-r1-search | 13.610719 | 0 | False |
| [20260907T213854.104888-minimap2-fetch-inputs](classic/iterations/20260907T213854.104888-minimap2-fetch-inputs/result.json) | minimap2-fetch-inputs | 3.823714 | 0 | False |
| [20260907T213857.983421-build-minimap2-index](classic/iterations/20260907T213857.983421-build-minimap2-index/result.json) | build-minimap2-index | 73.642920 | 0 | False |
| [20260907T213901.501968-measurement-blast-10-r1-verify](classic/iterations/20260907T213901.501968-measurement-blast-10-r1-verify/result.json) | measurement-blast-10-r1-verify | 0.364959 | 0 | False |
| [20260907T213901.875281-measurement-blast-10-r2-search](classic/iterations/20260907T213901.875281-measurement-blast-10-r2-search/result.json) | measurement-blast-10-r2-search | 13.555711 | 0 | False |
| [20260907T213915.432927-measurement-blast-10-r2-verify](classic/iterations/20260907T213915.432927-measurement-blast-10-r2-verify/result.json) | measurement-blast-10-r2-verify | 0.364763 | 0 | False |
| [20260907T213915.806714-measurement-blast-10-r3-search](classic/iterations/20260907T213915.806714-measurement-blast-10-r3-search/result.json) | measurement-blast-10-r3-search | 13.560481 | 0 | False |
| [20260907T213916.555133-measurement-bwa-26643-r2-samse](classic/iterations/20260907T213916.555133-measurement-bwa-26643-r2-samse/result.json) | measurement-bwa-26643-r2-samse | 22.164438 | 0 | False |
| [20260907T213929.369220-measurement-blast-10-r3-verify](classic/iterations/20260907T213929.369220-measurement-blast-10-r3-verify/result.json) | measurement-blast-10-r3-verify | 0.364777 | 0 | False |
| [20260907T213929.742754-measurement-blast-30-r1-search](classic/iterations/20260907T213929.742754-measurement-blast-30-r1-search/result.json) | measurement-blast-30-r1-search | 36.728317 | 0 | False |
| [20260907T213938.771362-measurement-bwa-26643-r2-verify](classic/iterations/20260907T213938.771362-measurement-bwa-26643-r2-verify/result.json) | measurement-bwa-26643-r2-verify | 9.483293 | 0 | False |
| [20260907T213948.467508-measurement-bwa-26643-r3-aln](classic/iterations/20260907T213948.467508-measurement-bwa-26643-r3-aln/result.json) | measurement-bwa-26643-r3-aln | 39.724684 | 0 | False |
| [20260907T214006.473253-measurement-blast-30-r1-verify](classic/iterations/20260907T214006.473253-measurement-blast-30-r1-verify/result.json) | measurement-blast-30-r1-verify | 0.414810 | 0 | False |
| [20260907T214006.897512-measurement-blast-30-r2-search](classic/iterations/20260907T214006.897512-measurement-blast-30-r2-search/result.json) | measurement-blast-30-r2-search | 36.744833 | 0 | False |
| [20260907T214012.081155-pilot-minimap2-100-r1-search](classic/iterations/20260907T214012.081155-pilot-minimap2-100-r1-search/result.json) | pilot-minimap2-100-r1-search | 4.876171 | 0 | False |
| [20260907T214016.976237-pilot-minimap2-100-r1-verify](classic/iterations/20260907T214016.976237-pilot-minimap2-100-r1-verify/result.json) | pilot-minimap2-100-r1-verify | 1.467079 | 0 | False |
| [20260907T214028.261785-measurement-bwa-26643-r3-samse](classic/iterations/20260907T214028.261785-measurement-bwa-26643-r3-samse/result.json) | measurement-bwa-26643-r3-samse | 22.468728 | 0 | False |
| [20260907T214043.644669-measurement-blast-30-r2-verify](classic/iterations/20260907T214043.644669-measurement-blast-30-r2-verify/result.json) | measurement-blast-30-r2-verify | 0.414831 | 0 | False |
| [20260907T214044.068617-measurement-blast-30-r3-search](classic/iterations/20260907T214044.068617-measurement-blast-30-r3-search/result.json) | measurement-blast-30-r3-search | 36.641840 | 0 | False |
| [20260907T214050.799757-measurement-bwa-26643-r3-verify](classic/iterations/20260907T214050.799757-measurement-bwa-26643-r3-verify/result.json) | measurement-bwa-26643-r3-verify | 9.334660 | 0 | False |
| [20260907T214120.712952-measurement-blast-30-r3-verify](classic/iterations/20260907T214120.712952-measurement-blast-30-r3-verify/result.json) | measurement-blast-30-r3-verify | 0.414957 | 0 | False |
| [20260907T214121.137263-measurement-blast-100-r1-search](classic/iterations/20260907T214121.137263-measurement-blast-100-r1-search/result.json) | measurement-blast-100-r1-search | 140.249343 | 0 | False |
| [20260907T214341.389125-measurement-blast-100-r1-verify](classic/iterations/20260907T214341.389125-measurement-blast-100-r1-verify/result.json) | measurement-blast-100-r1-verify | 0.514888 | 0 | False |
| [20260907T214341.913794-measurement-blast-100-r2-search](classic/iterations/20260907T214341.913794-measurement-blast-100-r2-search/result.json) | measurement-blast-100-r2-search | 140.183367 | 0 | False |
| [20260907T214602.099929-measurement-blast-100-r2-verify](classic/iterations/20260907T214602.099929-measurement-blast-100-r2-verify/result.json) | measurement-blast-100-r2-verify | 0.515021 | 0 | False |
| [20260907T214602.625137-measurement-blast-100-r3-search](classic/iterations/20260907T214602.625137-measurement-blast-100-r3-search/result.json) | measurement-blast-100-r3-search | 140.565352 | 0 | False |
| [20260907T214628.835626-measurement-minimap2-10-r1-search](classic/iterations/20260907T214628.835626-measurement-minimap2-10-r1-search/result.json) | measurement-minimap2-10-r1-search | 4.074106 | 0 | False |
| [20260907T214632.912904-measurement-minimap2-10-r1-verify](classic/iterations/20260907T214632.912904-measurement-minimap2-10-r1-verify/result.json) | measurement-minimap2-10-r1-verify | 0.765592 | 0 | False |
| [20260907T214633.687635-measurement-minimap2-10-r2-search](classic/iterations/20260907T214633.687635-measurement-minimap2-10-r2-search/result.json) | measurement-minimap2-10-r2-search | 4.174294 | 0 | False |
| [20260907T214637.863909-measurement-minimap2-10-r2-verify](classic/iterations/20260907T214637.863909-measurement-minimap2-10-r2-verify/result.json) | measurement-minimap2-10-r2-verify | 0.765562 | 0 | False |
| [20260907T214638.638054-measurement-minimap2-10-r3-search](classic/iterations/20260907T214638.638054-measurement-minimap2-10-r3-search/result.json) | measurement-minimap2-10-r3-search | 4.073740 | 0 | False |
| [20260907T214642.713763-measurement-minimap2-10-r3-verify](classic/iterations/20260907T214642.713763-measurement-minimap2-10-r3-verify/result.json) | measurement-minimap2-10-r3-verify | 0.765351 | 0 | False |
| [20260907T214643.488208-measurement-minimap2-30-r1-search](classic/iterations/20260907T214643.488208-measurement-minimap2-30-r1-search/result.json) | measurement-minimap2-30-r1-search | 4.224820 | 0 | False |
| [20260907T214646.690920-measurement-bwa-10-r1-aln](classic/iterations/20260907T214646.690920-measurement-bwa-10-r1-aln/result.json) | measurement-bwa-10-r1-aln | 1.066026 | 0 | False |
| [20260907T214647.715108-measurement-minimap2-30-r1-verify](classic/iterations/20260907T214647.715108-measurement-minimap2-30-r1-verify/result.json) | measurement-minimap2-30-r1-verify | 0.865576 | 0 | False |
| [20260907T214647.859177-measurement-bwa-10-r1-samse](classic/iterations/20260907T214647.859177-measurement-bwa-10-r1-samse/result.json) | measurement-bwa-10-r1-samse | 1.918684 | 0 | False |
| [20260907T214648.589612-measurement-minimap2-30-r2-search](classic/iterations/20260907T214648.589612-measurement-minimap2-30-r2-search/result.json) | measurement-minimap2-30-r2-search | 4.274519 | 0 | False |
| [20260907T214649.868760-measurement-bwa-10-r1-verify](classic/iterations/20260907T214649.868760-measurement-bwa-10-r1-verify/result.json) | measurement-bwa-10-r1-verify | 0.415007 | 0 | False |
| [20260907T214650.463789-measurement-bwa-10-r2-aln](classic/iterations/20260907T214650.463789-measurement-bwa-10-r2-aln/result.json) | measurement-bwa-10-r2-aln | 1.016149 | 0 | False |
| [20260907T214651.568970-measurement-bwa-10-r2-samse](classic/iterations/20260907T214651.568970-measurement-bwa-10-r2-samse/result.json) | measurement-bwa-10-r2-samse | 1.968082 | 0 | False |
| [20260907T214652.866346-measurement-minimap2-30-r2-verify](classic/iterations/20260907T214652.866346-measurement-minimap2-30-r2-verify/result.json) | measurement-minimap2-30-r2-verify | 0.865547 | 0 | False |
| [20260907T214653.625566-measurement-bwa-10-r2-verify](classic/iterations/20260907T214653.625566-measurement-bwa-10-r2-verify/result.json) | measurement-bwa-10-r2-verify | 0.414973 | 0 | False |
| [20260907T214653.741139-measurement-minimap2-30-r3-search](classic/iterations/20260907T214653.741139-measurement-minimap2-30-r3-search/result.json) | measurement-minimap2-30-r3-search | 4.225042 | 0 | False |
| [20260907T214654.220072-measurement-bwa-10-r3-aln](classic/iterations/20260907T214654.220072-measurement-bwa-10-r3-aln/result.json) | measurement-bwa-10-r3-aln | 1.015980 | 0 | False |
| [20260907T214655.323392-measurement-bwa-10-r3-samse](classic/iterations/20260907T214655.323392-measurement-bwa-10-r3-samse/result.json) | measurement-bwa-10-r3-samse | 1.918212 | 0 | False |
| [20260907T214657.331123-measurement-bwa-10-r3-verify](classic/iterations/20260907T214657.331123-measurement-bwa-10-r3-verify/result.json) | measurement-bwa-10-r3-verify | 0.414983 | 0 | False |
| [20260907T214657.923573-measurement-bwa-30-r1-aln](classic/iterations/20260907T214657.923573-measurement-bwa-30-r1-aln/result.json) | measurement-bwa-30-r1-aln | 1.066210 | 0 | False |
| [20260907T214657.968499-measurement-minimap2-30-r3-verify](classic/iterations/20260907T214657.968499-measurement-minimap2-30-r3-verify/result.json) | measurement-minimap2-30-r3-verify | 0.865713 | 0 | False |
| [20260907T214658.843529-measurement-minimap2-100-r1-search](classic/iterations/20260907T214658.843529-measurement-minimap2-100-r1-search/result.json) | measurement-minimap2-100-r1-search | 4.826538 | 0 | False |
| [20260907T214659.078775-measurement-bwa-30-r1-samse](classic/iterations/20260907T214659.078775-measurement-bwa-30-r1-samse/result.json) | measurement-bwa-30-r1-samse | 1.968205 | 0 | False |
| [20260907T214701.135595-measurement-bwa-30-r1-verify](classic/iterations/20260907T214701.135595-measurement-bwa-30-r1-verify/result.json) | measurement-bwa-30-r1-verify | 0.415037 | 0 | False |
| [20260907T214701.731015-measurement-bwa-30-r2-aln](classic/iterations/20260907T214701.731015-measurement-bwa-30-r2-aln/result.json) | measurement-bwa-30-r2-aln | 1.066408 | 0 | False |
| [20260907T214702.886742-measurement-bwa-30-r2-samse](classic/iterations/20260907T214702.886742-measurement-bwa-30-r2-samse/result.json) | measurement-bwa-30-r2-samse | 1.968639 | 0 | False |
| [20260907T214703.672495-measurement-minimap2-100-r1-verify](classic/iterations/20260907T214703.672495-measurement-minimap2-100-r1-verify/result.json) | measurement-minimap2-100-r1-verify | 1.466917 | 0 | False |
| [20260907T214704.944632-measurement-bwa-30-r2-verify](classic/iterations/20260907T214704.944632-measurement-bwa-30-r2-verify/result.json) | measurement-bwa-30-r2-verify | 0.415182 | 0 | False |
| [20260907T214705.149171-measurement-minimap2-100-r2-search](classic/iterations/20260907T214705.149171-measurement-minimap2-100-r2-search/result.json) | measurement-minimap2-100-r2-search | 4.826337 | 0 | False |
| [20260907T214705.539261-measurement-bwa-30-r3-aln](classic/iterations/20260907T214705.539261-measurement-bwa-30-r3-aln/result.json) | measurement-bwa-30-r3-aln | 1.066051 | 0 | False |
| [20260907T214706.694676-measurement-bwa-30-r3-samse](classic/iterations/20260907T214706.694676-measurement-bwa-30-r3-samse/result.json) | measurement-bwa-30-r3-samse | 1.968139 | 0 | False |
| [20260907T214708.754070-measurement-bwa-30-r3-verify](classic/iterations/20260907T214708.754070-measurement-bwa-30-r3-verify/result.json) | measurement-bwa-30-r3-verify | 0.414735 | 0 | False |
| [20260907T214709.978118-measurement-minimap2-100-r2-verify](classic/iterations/20260907T214709.978118-measurement-minimap2-100-r2-verify/result.json) | measurement-minimap2-100-r2-verify | 1.466596 | 0 | False |
| [20260907T214711.454877-measurement-minimap2-100-r3-search](classic/iterations/20260907T214711.454877-measurement-minimap2-100-r3-search/result.json) | measurement-minimap2-100-r3-search | 4.826276 | 0 | False |
| [20260907T214716.283956-measurement-minimap2-100-r3-verify](classic/iterations/20260907T214716.283956-measurement-minimap2-100-r3-verify/result.json) | measurement-minimap2-100-r3-verify | 1.466841 | 0 | False |
| [20260907T214717.761149-measurement-minimap2-1000-r1-search](classic/iterations/20260907T214717.761149-measurement-minimap2-1000-r1-search/result.json) | measurement-minimap2-1000-r1-search | 12.649133 | 0 | False |
| [20260907T214730.413213-measurement-minimap2-1000-r1-verify](classic/iterations/20260907T214730.413213-measurement-minimap2-1000-r1-verify/result.json) | measurement-minimap2-1000-r1-verify | 12.238725 | 0 | False |
| [20260907T214742.666500-measurement-minimap2-1000-r2-search](classic/iterations/20260907T214742.666500-measurement-minimap2-1000-r2-search/result.json) | measurement-minimap2-1000-r2-search | 12.648503 | 0 | False |
| [20260907T214755.318679-measurement-minimap2-1000-r2-verify](classic/iterations/20260907T214755.318679-measurement-minimap2-1000-r2-verify/result.json) | measurement-minimap2-1000-r2-verify | 12.188237 | 0 | False |
| [20260907T214800.253227-native-witness-tests](classic/iterations/20260907T214800.253227-native-witness-tests/result.json) | native-witness-tests | 4.880900 | 0 | False |
| [20260907T214805.151180-native-witness-build](classic/iterations/20260907T214805.151180-native-witness-build/result.json) | native-witness-build | 2.473144 | 0 | False |
| [20260907T214807.522283-measurement-minimap2-1000-r3-search](classic/iterations/20260907T214807.522283-measurement-minimap2-1000-r3-search/result.json) | measurement-minimap2-1000-r3-search | 12.649436 | 0 | False |
| [20260907T214807.700132-build-ooff-index-8](classic/iterations/20260907T214807.700132-build-ooff-index-8/result.json) | build-ooff-index-8 | 206.125300 | 0 | False |
| [20260907T214820.176411-measurement-minimap2-1000-r3-verify](classic/iterations/20260907T214820.176411-measurement-minimap2-1000-r3-verify/result.json) | measurement-minimap2-1000-r3-verify | 12.289903 | 0 | False |
| [20260907T214823.193454-measurement-blast-100-r3-verify](classic/iterations/20260907T214823.193454-measurement-blast-100-r3-verify/result.json) | measurement-blast-100-r3-verify | 0.515112 | 0 | False |
| [20260907T214823.718993-measurement-blast-1000-r1-search](classic/iterations/20260907T214823.718993-measurement-blast-1000-r1-search/result.json) | measurement-blast-1000-r1-search | 600.004070 | -15 | True |
| [20260907T214832.482890-measurement-minimap2-5000-r1-search](classic/iterations/20260907T214832.482890-measurement-minimap2-5000-r1-search/result.json) | measurement-minimap2-5000-r1-search | 69.208329 | 0 | False |
| [20260907T214941.696232-measurement-minimap2-5000-r1-verify](classic/iterations/20260907T214941.696232-measurement-minimap2-5000-r1-verify/result.json) | measurement-minimap2-5000-r1-verify | 96.676364 | 0 | False |
| [20260907T215118.405415-measurement-minimap2-5000-r2-search](classic/iterations/20260907T215118.405415-measurement-minimap2-5000-r2-search/result.json) | measurement-minimap2-5000-r2-search | 69.158484 | 0 | False |
| [20260907T215135.141795-archive-witness-source](classic/iterations/20260907T215135.141795-archive-witness-source/result.json) | archive-witness-source | 0.032127 | 0 | False |
| [20260907T215135.232304-parallel-human-gate-sassy-k0](classic/iterations/20260907T215135.232304-parallel-human-gate-sassy-k0/result.json) | parallel-human-gate-sassy-k0 | 45.193715 | 0 | False |
| [20260907T215220.435028-parallel-human-gate-ooff-k0](classic/iterations/20260907T215220.435028-parallel-human-gate-ooff-k0/result.json) | parallel-human-gate-ooff-k0 | 1.016659 | 0 | False |
| [20260907T215221.464815-parallel-human-gate-sassy-k1](classic/iterations/20260907T215221.464815-parallel-human-gate-sassy-k1/result.json) | parallel-human-gate-sassy-k1 | 19.479218 | 0 | False |
| [20260907T215227.571852-measurement-minimap2-5000-r2-verify](classic/iterations/20260907T215227.571852-measurement-minimap2-5000-r2-verify/result.json) | measurement-minimap2-5000-r2-verify | 97.564121 | 0 | False |
| [20260907T215240.946983-parallel-human-gate-ooff-k1](classic/iterations/20260907T215240.946983-parallel-human-gate-ooff-k1/result.json) | parallel-human-gate-ooff-k1 | 4.628778 | 0 | False |
| [20260907T215245.588888-parallel-human-gate-sassy-k2](classic/iterations/20260907T215245.588888-parallel-human-gate-sassy-k2/result.json) | parallel-human-gate-sassy-k2 | 19.678558 | 0 | False |
| [20260907T215305.270502-parallel-human-gate-ooff-k2](classic/iterations/20260907T215305.270502-parallel-human-gate-ooff-k2/result.json) | parallel-human-gate-ooff-k2 | 13.151413 | 0 | False |
| [20260907T215318.435477-parallel-human-gate-sassy-k3](classic/iterations/20260907T215318.435477-parallel-human-gate-sassy-k3/result.json) | parallel-human-gate-sassy-k3 | 20.180565 | 0 | False |
| [20260907T215338.619301-parallel-human-gate-ooff-k3](classic/iterations/20260907T215338.619301-parallel-human-gate-ooff-k3/result.json) | parallel-human-gate-ooff-k3 | 2.720979 | 0 | False |
| [20260907T215343.375896-pilot-ooff-100-r1-search](classic/iterations/20260907T215343.375896-pilot-ooff-100-r1-search/result.json) | pilot-ooff-100-r1-search | 0.164352 | 0 | False |
| [20260907T215343.555509-pilot-ooff-100-r1-verify](classic/iterations/20260907T215343.555509-pilot-ooff-100-r1-verify/result.json) | pilot-ooff-100-r1-verify | 0.465367 | 1 | False |
| [20260907T215405.173504-measurement-minimap2-5000-r3-search](classic/iterations/20260907T215405.173504-measurement-minimap2-5000-r3-search/result.json) | measurement-minimap2-5000-r3-search | 69.157267 | 0 | False |
| [20260907T215514.341832-measurement-minimap2-5000-r3-verify](classic/iterations/20260907T215514.341832-measurement-minimap2-5000-r3-verify/result.json) | measurement-minimap2-5000-r3-verify | 96.723023 | 0 | False |
| [20260907T215651.106705-measurement-minimap2-10000-r1-search](classic/iterations/20260907T215651.106705-measurement-minimap2-10000-r1-search/result.json) | measurement-minimap2-10000-r1-search | 138.602698 | 0 | False |
| [20260907T215751.735255-word-cache-tests](classic/iterations/20260907T215751.735255-word-cache-tests/result.json) | word-cache-tests | 2.872521 | 0 | False |
| [20260907T215754.611777-word-cache-build](classic/iterations/20260907T215754.611777-word-cache-build/result.json) | word-cache-build | 3.931658 | 0 | False |
| [20260907T215758.611053-word-cache-archive](classic/iterations/20260907T215758.611053-word-cache-archive/result.json) | word-cache-archive | 0.016035 | 0 | False |
| [20260907T215758.629650-word-cache-gate-baseline-k0](classic/iterations/20260907T215758.629650-word-cache-gate-baseline-k0/result.json) | word-cache-gate-baseline-k0 | 0.264454 | 0 | False |
| [20260907T215758.907509-word-cache-gate-cached-k0](classic/iterations/20260907T215758.907509-word-cache-gate-cached-k0/result.json) | word-cache-gate-cached-k0 | 0.264914 | 0 | False |
| [20260907T215759.186036-word-cache-gate-baseline-k1](classic/iterations/20260907T215759.186036-word-cache-gate-baseline-k1/result.json) | word-cache-gate-baseline-k1 | 0.265092 | 0 | False |
| [20260907T215759.464903-word-cache-gate-cached-k1](classic/iterations/20260907T215759.464903-word-cache-gate-cached-k1/result.json) | word-cache-gate-cached-k1 | 0.264663 | 0 | False |
| [20260907T215759.743321-word-cache-gate-baseline-k2](classic/iterations/20260907T215759.743321-word-cache-gate-baseline-k2/result.json) | word-cache-gate-baseline-k2 | 0.515641 | 0 | False |
| [20260907T215800.272603-word-cache-gate-cached-k2](classic/iterations/20260907T215800.272603-word-cache-gate-cached-k2/result.json) | word-cache-gate-cached-k2 | 0.515780 | 0 | False |
| [20260907T215800.802912-word-cache-gate-baseline-k3](classic/iterations/20260907T215800.802912-word-cache-gate-baseline-k3/result.json) | word-cache-gate-baseline-k3 | 1.017962 | 0 | False |
| [20260907T215801.834582-word-cache-gate-cached-k3](classic/iterations/20260907T215801.834582-word-cache-gate-cached-k3/result.json) | word-cache-gate-cached-k3 | 1.017516 | 0 | False |
| [20260907T215802.888374-word-cache-1000-baseline-r1](classic/iterations/20260907T215802.888374-word-cache-1000-baseline-r1/result.json) | word-cache-1000-baseline-r1 | 86.162009 | 0 | False |
| [20260907T215823.766147-measurement-blast-5000-r1-search](classic/iterations/20260907T215823.766147-measurement-blast-5000-r1-search/result.json) | measurement-blast-5000-r1-search | 600.012811 | -15 | True |
| [20260907T215909.756220-measurement-minimap2-10000-r1-verify](classic/iterations/20260907T215909.756220-measurement-minimap2-10000-r1-verify/result.json) | measurement-minimap2-10000-r1-verify | 232.659711 | 0 | False |
| [20260907T215929.073236-word-cache-1000-cached-r1](classic/iterations/20260907T215929.073236-word-cache-1000-cached-r1/result.json) | word-cache-1000-cached-r1 | 30.720870 | 0 | False |
| [20260907T215959.810094-word-cache-1000-cached-r2](classic/iterations/20260907T215959.810094-word-cache-1000-cached-r2/result.json) | word-cache-1000-cached-r2 | 27.762052 | 0 | False |
| [20260907T220027.586677-word-cache-1000-baseline-r2](classic/iterations/20260907T220027.586677-word-cache-1000-baseline-r2/result.json) | word-cache-1000-baseline-r2 | 27.558057 | 0 | False |
| [20260907T220055.159302-word-cache-1000-baseline-r3](classic/iterations/20260907T220055.159302-word-cache-1000-baseline-r3/result.json) | word-cache-1000-baseline-r3 | 27.610007 | 0 | False |
| [20260907T220122.783989-word-cache-1000-cached-r3](classic/iterations/20260907T220122.783989-word-cache-1000-cached-r3/result.json) | word-cache-1000-cached-r3 | 27.360263 | 0 | False |
| [20260907T220150.206205-pilot-ooff-100-r1-search](classic/iterations/20260907T220150.206205-pilot-ooff-100-r1-search/result.json) | pilot-ooff-100-r1-search | 0.164339 | 0 | False |
| [20260907T220150.386412-pilot-ooff-100-r1-verify](classic/iterations/20260907T220150.386412-pilot-ooff-100-r1-verify/result.json) | pilot-ooff-100-r1-verify | 0.515706 | 0 | False |
| [20260907T220150.924202-measurement-ooff-10-r1-search](classic/iterations/20260907T220150.924202-measurement-ooff-10-r1-search/result.json) | measurement-ooff-10-r1-search | 0.114151 | 0 | False |
| [20260907T220151.052532-measurement-ooff-10-r1-verify](classic/iterations/20260907T220151.052532-measurement-ooff-10-r1-verify/result.json) | measurement-ooff-10-r1-verify | 0.365032 | 0 | False |
| [20260907T220151.428873-measurement-ooff-10-r2-search](classic/iterations/20260907T220151.428873-measurement-ooff-10-r2-search/result.json) | measurement-ooff-10-r2-search | 0.114280 | 0 | False |
| [20260907T220151.557518-measurement-ooff-10-r2-verify](classic/iterations/20260907T220151.557518-measurement-ooff-10-r2-verify/result.json) | measurement-ooff-10-r2-verify | 0.365217 | 0 | False |
| [20260907T220151.934356-measurement-ooff-10-r3-search](classic/iterations/20260907T220151.934356-measurement-ooff-10-r3-search/result.json) | measurement-ooff-10-r3-search | 0.114829 | 0 | False |
| [20260907T220152.063517-measurement-ooff-10-r3-verify](classic/iterations/20260907T220152.063517-measurement-ooff-10-r3-verify/result.json) | measurement-ooff-10-r3-verify | 0.365019 | 0 | False |
| [20260907T220152.441515-measurement-ooff-30-r1-search](classic/iterations/20260907T220152.441515-measurement-ooff-30-r1-search/result.json) | measurement-ooff-30-r1-search | 0.114551 | 0 | False |
| [20260907T220152.570873-measurement-ooff-30-r1-verify](classic/iterations/20260907T220152.570873-measurement-ooff-30-r1-verify/result.json) | measurement-ooff-30-r1-verify | 0.365214 | 0 | False |
| [20260907T220152.948323-measurement-ooff-30-r2-search](classic/iterations/20260907T220152.948323-measurement-ooff-30-r2-search/result.json) | measurement-ooff-30-r2-search | 0.114414 | 0 | False |
| [20260907T220153.077555-measurement-ooff-30-r2-verify](classic/iterations/20260907T220153.077555-measurement-ooff-30-r2-verify/result.json) | measurement-ooff-30-r2-verify | 0.364757 | 0 | False |
| [20260907T220153.454869-measurement-ooff-30-r3-search](classic/iterations/20260907T220153.454869-measurement-ooff-30-r3-search/result.json) | measurement-ooff-30-r3-search | 0.114103 | 0 | False |
| [20260907T220153.583836-measurement-ooff-30-r3-verify](classic/iterations/20260907T220153.583836-measurement-ooff-30-r3-verify/result.json) | measurement-ooff-30-r3-verify | 0.364954 | 0 | False |
| [20260907T220153.961745-measurement-ooff-100-r1-search](classic/iterations/20260907T220153.961745-measurement-ooff-100-r1-search/result.json) | measurement-ooff-100-r1-search | 0.114201 | 0 | False |
| [20260907T220154.091197-measurement-ooff-100-r1-verify](classic/iterations/20260907T220154.091197-measurement-ooff-100-r1-verify/result.json) | measurement-ooff-100-r1-verify | 0.364911 | 0 | False |
| [20260907T220154.469350-measurement-ooff-100-r2-search](classic/iterations/20260907T220154.469350-measurement-ooff-100-r2-search/result.json) | measurement-ooff-100-r2-search | 0.114280 | 0 | False |
| [20260907T220154.598994-measurement-ooff-100-r2-verify](classic/iterations/20260907T220154.598994-measurement-ooff-100-r2-verify/result.json) | measurement-ooff-100-r2-verify | 0.415334 | 0 | False |
| [20260907T220155.027892-measurement-ooff-100-r3-search](classic/iterations/20260907T220155.027892-measurement-ooff-100-r3-search/result.json) | measurement-ooff-100-r3-search | 0.114424 | 0 | False |
| [20260907T220155.157489-measurement-ooff-100-r3-verify](classic/iterations/20260907T220155.157489-measurement-ooff-100-r3-verify/result.json) | measurement-ooff-100-r3-verify | 0.364834 | 0 | False |
| [20260907T220155.536127-measurement-ooff-1000-r1-search](classic/iterations/20260907T220155.536127-measurement-ooff-1000-r1-search/result.json) | measurement-ooff-1000-r1-search | 0.164232 | 0 | False |
| [20260907T220155.716235-measurement-ooff-1000-r1-verify](classic/iterations/20260907T220155.716235-measurement-ooff-1000-r1-verify/result.json) | measurement-ooff-1000-r1-verify | 0.465063 | 0 | False |
| [20260907T220156.199443-measurement-ooff-1000-r2-search](classic/iterations/20260907T220156.199443-measurement-ooff-1000-r2-search/result.json) | measurement-ooff-1000-r2-search | 0.164578 | 0 | False |
| [20260907T220156.381159-measurement-ooff-1000-r2-verify](classic/iterations/20260907T220156.381159-measurement-ooff-1000-r2-verify/result.json) | measurement-ooff-1000-r2-verify | 0.415315 | 0 | False |
| [20260907T220156.816223-measurement-ooff-1000-r3-search](classic/iterations/20260907T220156.816223-measurement-ooff-1000-r3-search/result.json) | measurement-ooff-1000-r3-search | 0.164535 | 0 | False |
| [20260907T220156.998363-measurement-ooff-1000-r3-verify](classic/iterations/20260907T220156.998363-measurement-ooff-1000-r3-verify/result.json) | measurement-ooff-1000-r3-verify | 0.415147 | 0 | False |
| [20260907T220157.434779-measurement-ooff-5000-r1-search](classic/iterations/20260907T220157.434779-measurement-ooff-5000-r1-search/result.json) | measurement-ooff-5000-r1-search | 0.214494 | 0 | False |
| [20260907T220157.669783-measurement-ooff-5000-r1-verify](classic/iterations/20260907T220157.669783-measurement-ooff-5000-r1-verify/result.json) | measurement-ooff-5000-r1-verify | 1.468030 | 0 | False |
| [20260907T220159.178730-measurement-ooff-5000-r2-search](classic/iterations/20260907T220159.178730-measurement-ooff-5000-r2-search/result.json) | measurement-ooff-5000-r2-search | 0.215443 | 0 | False |
| [20260907T220159.418181-measurement-ooff-5000-r2-verify](classic/iterations/20260907T220159.418181-measurement-ooff-5000-r2-verify/result.json) | measurement-ooff-5000-r2-verify | 0.516455 | 0 | False |
| [20260907T220159.980850-measurement-ooff-5000-r3-search](classic/iterations/20260907T220159.980850-measurement-ooff-5000-r3-search/result.json) | measurement-ooff-5000-r3-search | 0.165293 | 0 | False |
| [20260907T220200.173415-measurement-ooff-5000-r3-verify](classic/iterations/20260907T220200.173415-measurement-ooff-5000-r3-verify/result.json) | measurement-ooff-5000-r3-verify | 0.519239 | 0 | False |
| [20260907T220200.743740-measurement-ooff-10000-r1-search](classic/iterations/20260907T220200.743740-measurement-ooff-10000-r1-search/result.json) | measurement-ooff-10000-r1-search | 0.264979 | 0 | False |
| [20260907T220201.041315-measurement-ooff-10000-r1-verify](classic/iterations/20260907T220201.041315-measurement-ooff-10000-r1-verify/result.json) | measurement-ooff-10000-r1-verify | 1.218937 | 0 | False |
| [20260907T220202.341608-measurement-ooff-10000-r2-search](classic/iterations/20260907T220202.341608-measurement-ooff-10000-r2-search/result.json) | measurement-ooff-10000-r2-search | 0.215724 | 0 | False |
| [20260907T220202.596401-measurement-ooff-10000-r2-verify](classic/iterations/20260907T220202.596401-measurement-ooff-10000-r2-verify/result.json) | measurement-ooff-10000-r2-verify | 0.618220 | 0 | False |
| [20260907T220203.313969-measurement-ooff-10000-r3-search](classic/iterations/20260907T220203.313969-measurement-ooff-10000-r3-search/result.json) | measurement-ooff-10000-r3-search | 0.217190 | 0 | False |
| [20260907T220203.577562-measurement-ooff-10000-r3-verify](classic/iterations/20260907T220203.577562-measurement-ooff-10000-r3-verify/result.json) | measurement-ooff-10000-r3-verify | 0.618453 | 0 | False |
| [20260907T220204.301193-measurement-ooff-26643-r1-search](classic/iterations/20260907T220204.301193-measurement-ooff-26643-r1-search/result.json) | measurement-ooff-26643-r1-search | 0.365262 | 0 | False |
| [20260907T220204.726364-measurement-ooff-26643-r1-verify](classic/iterations/20260907T220204.726364-measurement-ooff-26643-r1-verify/result.json) | measurement-ooff-26643-r1-verify | 2.471379 | 0 | False |
| [20260907T220207.401558-measurement-ooff-26643-r2-search](classic/iterations/20260907T220207.401558-measurement-ooff-26643-r2-search/result.json) | measurement-ooff-26643-r2-search | 0.265726 | 0 | False |
| [20260907T220207.744997-measurement-ooff-26643-r2-verify](classic/iterations/20260907T220207.744997-measurement-ooff-26643-r2-verify/result.json) | measurement-ooff-26643-r2-verify | 0.967880 | 0 | False |
| [20260907T220208.947314-measurement-ooff-26643-r3-search](classic/iterations/20260907T220208.947314-measurement-ooff-26643-r3-search/result.json) | measurement-ooff-26643-r3-search | 0.264810 | 0 | False |
| [20260907T220209.310688-measurement-ooff-26643-r3-verify](classic/iterations/20260907T220209.310688-measurement-ooff-26643-r3-verify/result.json) | measurement-ooff-26643-r3-verify | 0.967355 | 0 | False |
| [20260907T220302.528030-measurement-minimap2-10000-r2-search](classic/iterations/20260907T220302.528030-measurement-minimap2-10000-r2-search/result.json) | measurement-minimap2-10000-r2-search | 143.564866 | 0 | False |
| [20260907T220425.415917-extension-reuse-tests](classic/iterations/20260907T220425.415917-extension-reuse-tests/result.json) | extension-reuse-tests | 2.972339 | 0 | False |
| [20260907T220428.496530-extension-reuse-build](classic/iterations/20260907T220428.496530-extension-reuse-build/result.json) | extension-reuse-build | 3.827094 | 0 | False |
| [20260907T220432.480061-extension-reuse-archive](classic/iterations/20260907T220432.480061-extension-reuse-archive/result.json) | extension-reuse-archive | 0.015859 | 0 | False |
| [20260907T220432.590959-extension-gate-retained-k0](classic/iterations/20260907T220432.590959-extension-gate-retained-k0/result.json) | extension-gate-retained-k0 | 0.265014 | 0 | False |
| [20260907T220432.960518-extension-gate-disabled-k0](classic/iterations/20260907T220432.960518-extension-gate-disabled-k0/result.json) | extension-gate-disabled-k0 | 0.265198 | 0 | False |
| [20260907T220433.332071-extension-gate-reuse-k0](classic/iterations/20260907T220433.332071-extension-gate-reuse-k0/result.json) | extension-gate-reuse-k0 | 0.265217 | 0 | False |
| [20260907T220433.700704-extension-gate-retained-k1](classic/iterations/20260907T220433.700704-extension-gate-retained-k1/result.json) | extension-gate-retained-k1 | 0.264788 | 0 | False |
| [20260907T220434.070324-extension-gate-disabled-k1](classic/iterations/20260907T220434.070324-extension-gate-disabled-k1/result.json) | extension-gate-disabled-k1 | 0.264963 | 0 | False |
| [20260907T220434.440924-extension-gate-reuse-k1](classic/iterations/20260907T220434.440924-extension-gate-reuse-k1/result.json) | extension-gate-reuse-k1 | 0.315221 | 0 | False |
| [20260907T220434.863331-extension-gate-retained-k2](classic/iterations/20260907T220434.863331-extension-gate-retained-k2/result.json) | extension-gate-retained-k2 | 0.314923 | 0 | False |
| [20260907T220435.284859-extension-gate-disabled-k2](classic/iterations/20260907T220435.284859-extension-gate-disabled-k2/result.json) | extension-gate-disabled-k2 | 0.365658 | 0 | False |
| [20260907T220435.755882-extension-gate-reuse-k2](classic/iterations/20260907T220435.755882-extension-gate-reuse-k2/result.json) | extension-gate-reuse-k2 | 0.365518 | 0 | False |
| [20260907T220436.227464-extension-gate-retained-k3](classic/iterations/20260907T220436.227464-extension-gate-retained-k3/result.json) | extension-gate-retained-k3 | 0.466385 | 0 | False |
| [20260907T220436.799832-extension-gate-disabled-k3](classic/iterations/20260907T220436.799832-extension-gate-disabled-k3/result.json) | extension-gate-disabled-k3 | 0.466679 | 0 | False |
| [20260907T220437.389369-extension-gate-reuse-k3](classic/iterations/20260907T220437.389369-extension-gate-reuse-k3/result.json) | extension-gate-reuse-k3 | 0.466206 | 0 | False |
| [20260907T220437.978401-extension-warmup-retained-r0](classic/iterations/20260907T220437.978401-extension-warmup-retained-r0/result.json) | extension-warmup-retained-r0 | 5.881743 | 0 | False |
| [20260907T220443.964914-extension-warmup-disabled-r0](classic/iterations/20260907T220443.964914-extension-warmup-disabled-r0/result.json) | extension-warmup-disabled-r0 | 5.931991 | 0 | False |
| [20260907T220450.002703-extension-warmup-reuse-r0](classic/iterations/20260907T220450.002703-extension-warmup-reuse-r0/result.json) | extension-warmup-reuse-r0 | 5.981304 | 0 | False |
| [20260907T220456.093183-extension-optimization-reuse-r1](classic/iterations/20260907T220456.093183-extension-optimization-reuse-r1/result.json) | extension-optimization-reuse-r1 | 5.983647 | 0 | False |
| [20260907T220502.184646-extension-optimization-retained-r1](classic/iterations/20260907T220502.184646-extension-optimization-retained-r1/result.json) | extension-optimization-retained-r1 | 5.830719 | 0 | False |
| [20260907T220508.123724-extension-optimization-disabled-r1](classic/iterations/20260907T220508.123724-extension-optimization-disabled-r1/result.json) | extension-optimization-disabled-r1 | 5.932048 | 0 | False |
| [20260907T220514.165565-extension-optimization-disabled-r2](classic/iterations/20260907T220514.165565-extension-optimization-disabled-r2/result.json) | extension-optimization-disabled-r2 | 5.882459 | 0 | False |
| [20260907T220520.155498-extension-optimization-reuse-r2](classic/iterations/20260907T220520.155498-extension-optimization-reuse-r2/result.json) | extension-optimization-reuse-r2 | 5.983299 | 0 | False |
| [20260907T220526.119591-measurement-minimap2-10000-r2-verify](classic/iterations/20260907T220526.119591-measurement-minimap2-10000-r2-verify/result.json) | measurement-minimap2-10000-r2-verify | 206.205802 | 0 | False |
| [20260907T220526.247074-extension-optimization-retained-r2](classic/iterations/20260907T220526.247074-extension-optimization-retained-r2/result.json) | extension-optimization-retained-r2 | 5.781118 | 0 | False |
| [20260907T220532.136435-extension-optimization-retained-r3](classic/iterations/20260907T220532.136435-extension-optimization-retained-r3/result.json) | extension-optimization-retained-r3 | 5.881444 | 0 | False |
| [20260907T220538.123359-extension-optimization-disabled-r3](classic/iterations/20260907T220538.123359-extension-optimization-disabled-r3/result.json) | extension-optimization-disabled-r3 | 5.931482 | 0 | False |
| [20260907T220544.162618-extension-optimization-reuse-r3](classic/iterations/20260907T220544.162618-extension-optimization-reuse-r3/result.json) | extension-optimization-reuse-r3 | 5.932666 | 0 | False |
| [20260907T220823.817452-measurement-blast-10000-r1-search](classic/iterations/20260907T220823.817452-measurement-blast-10000-r1-search/result.json) | measurement-blast-10000-r1-search | 600.004030 | -15 | True |
| [20260907T220852.419940-measurement-minimap2-10000-r3-search](classic/iterations/20260907T220852.419940-measurement-minimap2-10000-r3-search/result.json) | measurement-minimap2-10000-r3-search | 145.966236 | 0 | False |
| [20260907T221118.581001-measurement-minimap2-10000-r3-verify](classic/iterations/20260907T221118.581001-measurement-minimap2-10000-r3-verify/result.json) | measurement-minimap2-10000-r3-verify | 206.050679 | 0 | False |
| [20260907T221444.850377-measurement-minimap2-26643-r1-search](classic/iterations/20260907T221444.850377-measurement-minimap2-26643-r1-search/result.json) | measurement-minimap2-26643-r1-search | unavailable | None | False |
| [20260907T221823.829420-measurement-blast-26643-r1-search](classic/iterations/20260907T221823.829420-measurement-blast-26643-r1-search/result.json) | measurement-blast-26643-r1-search | 600.001698 | -15 | True |
| [20260907T222537.437769-additional-storage-check](classic/iterations/20260907T222537.437769-additional-storage-check/result.json) | additional-storage-check | 0.003559 | 0 | False |
| [20260907T222625.844975-measurement-minimap2-26643-r1-search](classic/iterations/20260907T222625.844975-measurement-minimap2-26643-r1-search/result.json) | measurement-minimap2-26643-r1-search | 455.338407 | 0 | False |
| [20260907T223401.234133-measurement-minimap2-26643-r1-verify](classic/iterations/20260907T223401.234133-measurement-minimap2-26643-r1-verify/result.json) | measurement-minimap2-26643-r1-verify | 563.475867 | 0 | False |
| [20260907T223914.765954-timeout_test-bwa-1-r1-aln](classic/iterations/20260907T223914.765954-timeout_test-bwa-1-r1-aln/result.json) | timeout_test-bwa-1-r1-aln | 0.516059 | 0 | False |
| [20260907T223915.625457-timeout_test-bwa-1-r1-samse](classic/iterations/20260907T223915.625457-timeout_test-bwa-1-r1-samse/result.json) | timeout_test-bwa-1-r1-samse | 0.042389 | -15 | True |
| [20260907T223942.201313-measurement-ooff-100000-r1-search](classic/iterations/20260907T223942.201313-measurement-ooff-100000-r1-search/result.json) | measurement-ooff-100000-r1-search | 42.177265 | 0 | False |
| [20260907T223946.316019-measurement-bwa-100000-r1-aln](classic/iterations/20260907T223946.316019-measurement-bwa-100000-r1-aln/result.json) | measurement-bwa-100000-r1-aln | 144.237612 | 0 | False |
| [20260907T223950.659732-measurement-blast-100000-r1-search](classic/iterations/20260907T223950.659732-measurement-blast-100000-r1-search/result.json) | measurement-blast-100000-r1-search | 600.016508 | -15 | True |
| [20260907T224024.596632-measurement-ooff-100000-r1-verify](classic/iterations/20260907T224024.596632-measurement-ooff-100000-r1-verify/result.json) | measurement-ooff-100000-r1-verify | 9.487559 | 0 | False |
| [20260907T224034.779087-measurement-ooff-100000-r2-search](classic/iterations/20260907T224034.779087-measurement-ooff-100000-r2-search/result.json) | measurement-ooff-100000-r2-search | 0.567232 | 0 | False |
| [20260907T224035.573574-measurement-ooff-100000-r2-verify](classic/iterations/20260907T224035.573574-measurement-ooff-100000-r2-verify/result.json) | measurement-ooff-100000-r2-verify | 2.624141 | 0 | False |
| [20260907T224039.031624-measurement-ooff-100000-r3-search](classic/iterations/20260907T224039.031624-measurement-ooff-100000-r3-search/result.json) | measurement-ooff-100000-r3-search | 0.517334 | 0 | False |
| [20260907T224039.863071-measurement-ooff-100000-r3-verify](classic/iterations/20260907T224039.863071-measurement-ooff-100000-r3-verify/result.json) | measurement-ooff-100000-r3-verify | 2.572589 | 0 | False |
| [20260907T224210.786995-measurement-bwa-100000-r1-samse](classic/iterations/20260907T224210.786995-measurement-bwa-100000-r1-samse/result.json) | measurement-bwa-100000-r1-samse | 99.472587 | 0 | False |
| [20260907T224324.894473-measurement-minimap2-26643-r2-search](classic/iterations/20260907T224324.894473-measurement-minimap2-26643-r2-search/result.json) | measurement-minimap2-26643-r2-search | 456.591988 | 0 | False |
| [20260907T224350.346878-measurement-bwa-100000-r1-verify](classic/iterations/20260907T224350.346878-measurement-bwa-100000-r1-verify/result.json) | measurement-bwa-100000-r1-verify | 39.046437 | 0 | False |
| [20260907T224430.005962-measurement-bwa-100000-r2-aln](classic/iterations/20260907T224430.005962-measurement-bwa-100000-r2-aln/result.json) | measurement-bwa-100000-r2-aln | 127.788477 | 0 | False |
| [20260907T224637.955125-measurement-bwa-100000-r2-samse](classic/iterations/20260907T224637.955125-measurement-bwa-100000-r2-samse/result.json) | measurement-bwa-100000-r2-samse | 87.835107 | 0 | False |
| [20260907T224805.947771-measurement-bwa-100000-r2-verify](classic/iterations/20260907T224805.947771-measurement-bwa-100000-r2-verify/result.json) | measurement-bwa-100000-r2-verify | 30.831994 | 0 | False |
| [20260907T224837.513194-measurement-bwa-100000-r3-aln](classic/iterations/20260907T224837.513194-measurement-bwa-100000-r3-aln/result.json) | measurement-bwa-100000-r3-aln | 127.774408 | 0 | False |
| [20260907T225045.541936-measurement-bwa-100000-r3-samse](classic/iterations/20260907T225045.541936-measurement-bwa-100000-r3-samse/result.json) | measurement-bwa-100000-r3-samse | 88.489090 | 0 | False |
| [20260907T225101.555853-measurement-minimap2-26643-r2-verify](classic/iterations/20260907T225101.555853-measurement-minimap2-26643-r2-verify/result.json) | measurement-minimap2-26643-r2-verify | 248.631734 | -15 | False |
| [20260907T225214.270267-measurement-bwa-100000-r3-verify](classic/iterations/20260907T225214.270267-measurement-bwa-100000-r3-verify/result.json) | measurement-bwa-100000-r3-verify | 30.634877 | 0 | False |
| [20260907T225513.797152-measurement-minimap2-100000-r1-search](classic/iterations/20260907T225513.797152-measurement-minimap2-100000-r1-search/result.json) | measurement-minimap2-100000-r1-search | 421.161202 | 137 | False |
| [20260907T233155.699248-build-sassy-classic-screen](classic/iterations/20260907T233155.699248-build-sassy-classic-screen/result.json) | build-sassy-classic-screen | 23.407296 | 0 | False |
| [20260907T233300.698338-test-sassy-screen-oracle](classic/iterations/20260907T233300.698338-test-sassy-screen-oracle/result.json) | test-sassy-screen-oracle | 0.164184 | 0 | False |
| [20260907T233447.268082-pilot-sassy-100-r1-search](classic/iterations/20260907T233447.268082-pilot-sassy-100-r1-search/result.json) | pilot-sassy-100-r1-search | 18.119341 | 0 | False |
| [20260907T233505.777525-pilot-sassy-100-r1-verify](classic/iterations/20260907T233505.777525-pilot-sassy-100-r1-verify/result.json) | pilot-sassy-100-r1-verify | 0.465334 | 0 | False |
| [20260907T233616.660962-measurement-sassy-10-r1-search](classic/iterations/20260907T233616.660962-measurement-sassy-10-r1-search/result.json) | measurement-sassy-10-r1-search | 1.768935 | 0 | False |
| [20260907T233618.829128-measurement-sassy-10-r1-verify](classic/iterations/20260907T233618.829128-measurement-sassy-10-r1-verify/result.json) | measurement-sassy-10-r1-verify | 0.365276 | 0 | False |
| [20260907T233619.943218-measurement-sassy-10-r2-search](classic/iterations/20260907T233619.943218-measurement-sassy-10-r2-search/result.json) | measurement-sassy-10-r2-search | 1.770050 | 0 | False |
| [20260907T233622.085476-measurement-sassy-10-r2-verify](classic/iterations/20260907T233622.085476-measurement-sassy-10-r2-verify/result.json) | measurement-sassy-10-r2-verify | 0.365466 | 0 | False |
| [20260907T233623.186570-measurement-sassy-10-r3-search](classic/iterations/20260907T233623.186570-measurement-sassy-10-r3-search/result.json) | measurement-sassy-10-r3-search | 1.769192 | 0 | False |
| [20260907T233625.312144-measurement-sassy-10-r3-verify](classic/iterations/20260907T233625.312144-measurement-sassy-10-r3-verify/result.json) | measurement-sassy-10-r3-verify | 0.365221 | 0 | False |
| [20260907T233626.383062-measurement-sassy-100-r1-search](classic/iterations/20260907T233626.383062-measurement-sassy-100-r1-search/result.json) | measurement-sassy-100-r1-search | 2.070623 | 0 | False |
| [20260907T233628.805410-measurement-sassy-100-r1-verify](classic/iterations/20260907T233628.805410-measurement-sassy-100-r1-verify/result.json) | measurement-sassy-100-r1-verify | 0.364918 | 0 | False |
| [20260907T233629.866870-measurement-sassy-100-r2-search](classic/iterations/20260907T233629.866870-measurement-sassy-100-r2-search/result.json) | measurement-sassy-100-r2-search | 2.020084 | 0 | False |
| [20260907T233632.233982-measurement-sassy-100-r2-verify](classic/iterations/20260907T233632.233982-measurement-sassy-100-r2-verify/result.json) | measurement-sassy-100-r2-verify | 0.365339 | 0 | False |
| [20260907T233633.289577-measurement-sassy-100-r3-search](classic/iterations/20260907T233633.289577-measurement-sassy-100-r3-search/result.json) | measurement-sassy-100-r3-search | 2.020198 | 0 | False |
| [20260907T233635.660233-measurement-sassy-100-r3-verify](classic/iterations/20260907T233635.660233-measurement-sassy-100-r3-verify/result.json) | measurement-sassy-100-r3-verify | 0.365542 | 0 | False |
| [20260907T233636.715440-measurement-sassy-1000-r1-search](classic/iterations/20260907T233636.715440-measurement-sassy-1000-r1-search/result.json) | measurement-sassy-1000-r1-search | 2.672151 | 0 | False |
| [20260907T233639.735939-measurement-sassy-1000-r1-verify](classic/iterations/20260907T233639.735939-measurement-sassy-1000-r1-verify/result.json) | measurement-sassy-1000-r1-verify | 0.365424 | 0 | False |
| [20260907T233640.801992-measurement-sassy-1000-r2-search](classic/iterations/20260907T233640.801992-measurement-sassy-1000-r2-search/result.json) | measurement-sassy-1000-r2-search | 2.721832 | 0 | False |
| [20260907T233643.870404-measurement-sassy-1000-r2-verify](classic/iterations/20260907T233643.870404-measurement-sassy-1000-r2-verify/result.json) | measurement-sassy-1000-r2-verify | 0.365021 | 0 | False |
| [20260907T233644.940363-measurement-sassy-1000-r3-search](classic/iterations/20260907T233644.940363-measurement-sassy-1000-r3-search/result.json) | measurement-sassy-1000-r3-search | 2.671692 | 0 | False |
| [20260907T233647.959757-measurement-sassy-1000-r3-verify](classic/iterations/20260907T233647.959757-measurement-sassy-1000-r3-verify/result.json) | measurement-sassy-1000-r3-verify | 0.415835 | 0 | False |
| [20260907T233649.084145-measurement-sassy-10000-r1-search](classic/iterations/20260907T233649.084145-measurement-sassy-10000-r1-search/result.json) | measurement-sassy-10000-r1-search | 3.823165 | 0 | False |
| [20260907T233653.256457-measurement-sassy-10000-r1-verify](classic/iterations/20260907T233653.256457-measurement-sassy-10000-r1-verify/result.json) | measurement-sassy-10000-r1-verify | 0.566035 | 0 | False |
| [20260907T233654.547930-measurement-sassy-10000-r2-search](classic/iterations/20260907T233654.547930-measurement-sassy-10000-r2-search/result.json) | measurement-sassy-10000-r2-search | 3.723346 | 0 | False |
| [20260907T233658.631034-measurement-sassy-10000-r2-verify](classic/iterations/20260907T233658.631034-measurement-sassy-10000-r2-verify/result.json) | measurement-sassy-10000-r2-verify | 0.566505 | 0 | False |
| [20260907T233659.947433-measurement-sassy-10000-r3-search](classic/iterations/20260907T233659.947433-measurement-sassy-10000-r3-search/result.json) | measurement-sassy-10000-r3-search | 3.726337 | 0 | False |
| [20260907T233704.030880-measurement-sassy-10000-r3-verify](classic/iterations/20260907T233704.030880-measurement-sassy-10000-r3-verify/result.json) | measurement-sassy-10000-r3-verify | 0.570572 | 0 | False |
| [20260907T233705.370059-measurement-sassy-100000-r1-search](classic/iterations/20260907T233705.370059-measurement-sassy-100000-r1-search/result.json) | measurement-sassy-100000-r1-search | 16.946374 | 0 | False |
| [20260907T233722.678668-measurement-sassy-100000-r1-verify](classic/iterations/20260907T233722.678668-measurement-sassy-100000-r1-verify/result.json) | measurement-sassy-100000-r1-verify | 2.478360 | 0 | False |
| [20260907T233726.292330-measurement-sassy-100000-r2-search](classic/iterations/20260907T233726.292330-measurement-sassy-100000-r2-search/result.json) | measurement-sassy-100000-r2-search | 16.746860 | 0 | False |
| [20260907T233743.486688-measurement-sassy-100000-r2-verify](classic/iterations/20260907T233743.486688-measurement-sassy-100000-r2-verify/result.json) | measurement-sassy-100000-r2-verify | 2.522220 | 0 | False |
| [20260907T233747.307286-measurement-sassy-100000-r3-search](classic/iterations/20260907T233747.307286-measurement-sassy-100000-r3-search/result.json) | measurement-sassy-100000-r3-search | 16.850495 | 0 | False |
| [20260907T233804.658170-measurement-sassy-100000-r3-verify](classic/iterations/20260907T233804.658170-measurement-sassy-100000-r3-verify/result.json) | measurement-sassy-100000-r3-verify | 2.472500 | 0 | False |

## Composite screening runs

Elapsed time includes all mapper/conversion/verification stages; pilot runs are labelled.

| Tool | ASOs | Rep | Phase | Threads | Seconds | Recovered | Complete |
| --- | ---: | ---: | --- | ---: | ---: | ---: | --- |
| blast | 100 | 1 | pilot | 8 | 141.449913 |  | False |
| bwa | 100 | 1 | pilot | 8 | 3.601127 | 100 | True |
| bwa | 100 | 1 | measurement | 8 | 3.549286 | 100 | True |
| bwa | 100 | 2 | measurement | 8 | 3.298796 | 100 | True |
| bwa | 100 | 3 | measurement | 8 | 3.299166 | 100 | True |
| bwa | 1000 | 1 | measurement | 8 | 4.302674 | 998 | True |
| bwa | 1000 | 2 | measurement | 8 | 4.151745 | 998 | True |
| bwa | 1000 | 3 | measurement | 8 | 4.152519 | 998 | True |
| bwa | 5000 | 1 | measurement | 8 | 15.729951 | 4924 | True |
| bwa | 5000 | 2 | measurement | 8 | 15.880905 | 4924 | True |
| bwa | 5000 | 3 | measurement | 8 | 15.632204 | 4924 | True |
| bwa | 10000 | 1 | measurement | 8 | 27.161440 | 9776 | True |
| bwa | 10000 | 2 | measurement | 8 | 26.961034 | 9776 | True |
| bwa | 10000 | 3 | measurement | 8 | 27.159314 | 9776 | True |
| bwa | 26643 | 1 | measurement | 8 | 72.327262 | 26019 | True |
| bwa | 26643 | 2 | measurement | 8 | 72.024910 | 26019 | True |
| blast | 10 | 1 | measurement | 8 | 13.975677 | 10 | True |
| blast | 10 | 2 | measurement | 8 | 13.920474 | 10 | True |
| blast | 10 | 3 | measurement | 8 | 13.925257 | 10 | True |
| blast | 30 | 1 | measurement | 8 | 37.143128 | 30 | True |
| bwa | 26643 | 3 | measurement | 8 | 71.528073 | 26019 | True |
| blast | 30 | 2 | measurement | 8 | 37.159664 | 30 | True |
| minimap2 | 100 | 1 | pilot | 8 | 6.343251 | 100 | True |
| blast | 30 | 3 | measurement | 8 | 37.056797 | 30 | True |
| blast | 100 | 1 | measurement | 8 | 140.764231 | 100 | True |
| blast | 100 | 2 | measurement | 8 | 140.698388 | 100 | True |
| blast | 100 | 3 | measurement | 8 | 141.080463 | 100 | True |
| minimap2 | 10 | 1 | measurement | 8 | 4.839698 | 10 | True |
| minimap2 | 10 | 2 | measurement | 8 | 4.939856 | 10 | True |
| minimap2 | 10 | 3 | measurement | 8 | 4.839091 | 10 | True |
| minimap2 | 30 | 1 | measurement | 8 | 5.090396 | 30 | True |
| bwa | 10 | 1 | measurement | 8 | 3.399716 | 10 | True |
| minimap2 | 30 | 2 | measurement | 8 | 5.140066 | 30 | True |
| bwa | 10 | 2 | measurement | 8 | 3.399204 | 10 | True |
| minimap2 | 30 | 3 | measurement | 8 | 5.090756 | 30 | True |
| bwa | 10 | 3 | measurement | 8 | 3.349175 | 10 | True |
| bwa | 30 | 1 | measurement | 8 | 3.449452 | 30 | True |
| minimap2 | 100 | 1 | measurement | 8 | 6.293455 | 100 | True |
| bwa | 30 | 2 | measurement | 8 | 3.450229 | 30 | True |
| minimap2 | 100 | 2 | measurement | 8 | 6.292933 | 100 | True |
| bwa | 30 | 3 | measurement | 8 | 3.448924 | 30 | True |
| minimap2 | 100 | 3 | measurement | 8 | 6.293117 | 100 | True |
| minimap2 | 1000 | 1 | measurement | 8 | 24.887858 | 970 | True |
| minimap2 | 1000 | 2 | measurement | 8 | 24.836740 | 970 | True |
| minimap2 | 1000 | 3 | measurement | 8 | 24.939339 | 970 | True |
| blast | 1000 | 1 | measurement | 8 | 600.004070 |  | False |
| minimap2 | 5000 | 1 | measurement | 8 | 165.884693 | 4808 | True |
| minimap2 | 5000 | 2 | measurement | 8 | 166.722604 | 4808 | True |
| ooff | 100 | 1 | pilot | 8 | 0.629719 |  | False |
| minimap2 | 5000 | 3 | measurement | 8 | 165.880290 | 4808 | True |
| minimap2 | 10000 | 1 | measurement | 8 | 371.262409 | 9512 | True |
| blast | 5000 | 1 | measurement | 8 | 600.012811 |  | False |
| ooff | 100 | 1 | pilot | 8 | 0.680046 | 100 | True |
| ooff | 10 | 1 | measurement | 8 | 0.479183 | 10 | True |
| ooff | 10 | 2 | measurement | 8 | 0.479497 | 10 | True |
| ooff | 10 | 3 | measurement | 8 | 0.479849 | 10 | True |
| ooff | 30 | 1 | measurement | 8 | 0.479765 | 30 | True |
| ooff | 30 | 2 | measurement | 8 | 0.479171 | 30 | True |
| ooff | 30 | 3 | measurement | 8 | 0.479058 | 30 | True |
| ooff | 100 | 1 | measurement | 8 | 0.479113 | 100 | True |
| ooff | 100 | 2 | measurement | 8 | 0.529614 | 100 | True |
| ooff | 100 | 3 | measurement | 8 | 0.479258 | 100 | True |
| ooff | 1000 | 1 | measurement | 8 | 0.629295 | 1000 | True |
| ooff | 1000 | 2 | measurement | 8 | 0.579893 | 1000 | True |
| ooff | 1000 | 3 | measurement | 8 | 0.579682 | 1000 | True |
| ooff | 5000 | 1 | measurement | 8 | 1.682525 | 5000 | True |
| ooff | 5000 | 2 | measurement | 8 | 0.731898 | 5000 | True |
| ooff | 5000 | 3 | measurement | 8 | 0.684532 | 5000 | True |
| ooff | 10000 | 1 | measurement | 8 | 1.483916 | 10000 | True |
| ooff | 10000 | 2 | measurement | 8 | 0.833944 | 10000 | True |
| ooff | 10000 | 3 | measurement | 8 | 0.835643 | 10000 | True |
| ooff | 26643 | 1 | measurement | 8 | 2.836641 | 26643 | True |
| ooff | 26643 | 2 | measurement | 8 | 1.233606 | 26643 | True |
| ooff | 26643 | 3 | measurement | 8 | 1.232164 | 26643 | True |
| minimap2 | 10000 | 2 | measurement | 8 | 349.770668 | 9512 | True |
| blast | 10000 | 1 | measurement | 8 | 600.004030 |  | False |
| minimap2 | 10000 | 3 | measurement | 8 | 352.016915 | 9512 | True |
| blast | 26643 | 1 | measurement | 8 | 600.001698 |  | False |
| minimap2 | 26643 | 1 | measurement | 8 | 1018.814274 | 25359 | True |
| bwa | 1 | 1 | timeout_test | 8 | 0.558448 |  | False |
| ooff | 100000 | 1 | measurement | 8 | 51.664824 | 100000 | True |
| bwa | 100000 | 1 | measurement | 8 | 282.756637 | 98774 | True |
| blast | 100000 | 1 | measurement | 8 | 600.016508 |  | False |
| ooff | 100000 | 2 | measurement | 8 | 3.191373 | 100000 | True |
| ooff | 100000 | 3 | measurement | 8 | 3.089923 | 100000 | True |
| minimap2 | 26643 | 2 | measurement | 8 | 705.223722 |  | False |
| bwa | 100000 | 2 | measurement | 8 | 246.455578 | 98774 | True |
| bwa | 100000 | 3 | measurement | 8 | 246.898375 | 98774 | True |
| minimap2 | 100000 | 1 | measurement | 8 | 421.161202 |  | False |
| sassy | 100 | 1 | pilot | 8 | 18.584675 | 100 | True |
| sassy | 10 | 1 | measurement | 8 | 2.134210 | 10 | True |
| sassy | 10 | 2 | measurement | 8 | 2.135517 | 10 | True |
| sassy | 10 | 3 | measurement | 8 | 2.134414 | 10 | True |
| sassy | 100 | 1 | measurement | 8 | 2.435541 | 100 | True |
| sassy | 100 | 2 | measurement | 8 | 2.385423 | 100 | True |
| sassy | 100 | 3 | measurement | 8 | 2.385740 | 100 | True |
| sassy | 1000 | 1 | measurement | 8 | 3.037575 | 1000 | True |
| sassy | 1000 | 2 | measurement | 8 | 3.086854 | 1000 | True |
| sassy | 1000 | 3 | measurement | 8 | 3.087527 | 1000 | True |
| sassy | 10000 | 1 | measurement | 8 | 4.389200 | 10000 | True |
| sassy | 10000 | 2 | measurement | 8 | 4.289851 | 10000 | True |
| sassy | 10000 | 3 | measurement | 8 | 4.296909 | 10000 | True |
| sassy | 100000 | 1 | measurement | 8 | 19.424734 | 100000 | True |
| sassy | 100000 | 2 | measurement | 8 | 19.269080 | 100000 | True |
| sassy | 100000 | 3 | measurement | 8 | 19.322996 | 100000 | True |
