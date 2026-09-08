## 2026-09-08 17:00 UTC — compact index and OligoAI shipping work

Latest user steering: fix Pi memory or move OligoAI to EC2. Compact sampled-SA16
format implemented and exact-position/rank equivalence tested, including unknown
bases, separators, boundaries and mapped CLI summary/screen/report output.
`oofft-index compact` converts immutable full indexes without rebuilding their SA.
Forward human index is 2,896,537,944 bytes; EC2 conversion 97.061 seconds.
Pi 5 four-thread 1,000 evenly spaced SCN2A ASOs: 75.214347 s, 3,147,152 KiB
peak RSS, all 1,000 query-summary rows identical to EC2 full/compact results.
No full-gene Pi latency/RSS guarantee. Reference is Ensembl110 gene bodies plus
transcripts, NOT intergenic whole-chromosome sequence. Every attempt in summary
ledger; EC2 compact sample concurrent with download is labelled diagnostic.

EC2 instance i-0b5bad3102a5345c5 is currently RUNNING at 13.40.47.242 (old IPs
stale), auto shutdown 19:43 UTC. Retained scratch mounted /mnt/oofft-walk by UUID.
New checkout /home/ubuntu/oofft-compact-20260908, binaries
/home/ubuntu/oofft-compact-target/release, index /home/ubuntu/ooff/data/fm-compact16-v1.
Local reference/index/cache: data/reference-compact-v1. Download SHA checked.
`scripts/relocate-index.py` verifies SHA before rebinding a copied reference path.

OligoAI changes STAGED ONLY at /tmp/oligoai-oofft-integration, actual repo
/home/barneyh/oligoai-v2. Preserve its pre-existing dirty server.ts, results-db.ts,
public/oligoai/design.js and design.html. Adapter invokes native CLI once, RCs and
deduplicates targets, excludes intended ENSG IDs, retains bins and completeness,
rejects failed/truncated output. Database/UI/CSV updated in staging. Bun bundle,
adapter failure/orientation tests and DB roundtrip tests pass. Live oligoai.service
is system service, user barneyh, PID759, 110MB RSS; restart must avoid active jobs.
Async deployment choice pending: keep web app Pi and EC2 counts (recommended),
all-local compact, or all EC2. No live edits/restart yet. No cloud migration yet.

Publication explicitly authorized. Main workspace .git read-only; commit/push
via isolated /tmp clone. No gh installed; git SSH works, public GitHub API works.
Repo barneyhill/ooff main c0c61a97bbb8df642ac2e103f112127690173ee1; no release tags;
crates.io oofft name unclaimed on check. Release workflow on v* runs CI, publishes
using existing CRATES_IO_API secret, creates GitHub release/binaries/checksums.
Do NOT use or expose old pasted token. Cargo package offline succeeded. Cargo
license-file now LICENSES.md with MIT and full ViennaRNA parameter terms; binary
archives retain notices too. Full tests/clippy currently finishing; not committed,
pushed, published or integrated yet. Earlier historical notes below can be stale.

# Native SCN2A summary complete — 2026-09-08 ~16:19 UTC

New defaultsummaryfull114889ASOwalk completed438.953321720s (7m19s),8threads,
m7i.2xlarge32GiB. Output28,812,047bytes; peakRSS13,789,412KiB includingmmap.
359,039,833totalASO/genomicsitepairs; bins0..3=[16189,341556,12851473,345830615].
Indexload.361302350s,search/output437.894896132s. RetainedOS cache,notcoldrun.
NoDDG,noindividualsiteoutput. All43localtests and x86summary/indexedtests passed.
Newpipeline is default; --genes optin IDs; --sites/explicitreport optin detailed.

Results retrieved to benchmarks/summary/iterations/20260908T161125-scn2a-genomic-default,
includingfullstdout(localgitignored),resources,commands,hashes. CSV/MDrefreshed.
Remote source/binary/results retained in summarycheckout/target as below.
Oldexternalbenchmark CANCELLED peruser; its215completedpartsretained only as
historicalartifacts. Do not restart it. EC2 stop requested after retrieval;
check finalstate if needed. Volumes retained, including /mnt/oofft-walk scratch.
Currentdocs docs/SUMMARY.md; no sourcecommit/push performed.

# Native summary mode — 2026-09-08 16:11 UTC

User explicitly cancelled the obsolete external full-report/ddG benchmark.
Stopped dispatcher1333 and descendants on18.175.151.21, preserving215completed
results. Do NOT resume that wrapper or its deferred verification. User no longer
wants ddG for the OligoAI default. Distinct genomic sites explicitly confirmed.

New CLI defaults to summary; --genes adds IDs, --sites/explicitreport preserves
site reports, --count-unit record-interval provides non-deduplicated counts.
Counts use edit-distance bins0..3, higher-than-k binsnull; unknownbasesincomplete.
Default genomic key contig/strand/orderedblocks (adjacentcoalesced), minimumcost
acrossrecords. Native fixedworkerpool sharedindex, bounded2*threadsqueues with
ordered output; one query dedup state perworker, no perhitserialization/traceback.
No newddgflag.43localtests inclliveViennapassed; strictclippy passed.

EC2 summarysource /home/ubuntu/oofft-summary-20260908; separate target
/home/ubuntu/oofft-summary-target. Sourcearchive /tmp/oofft-summary-source.tar.gz
(local), /home/ubuntu/oofft-summary-source.tar.gz(remote), SHA256
8fc266c29939c8d5cd4539b4c40765c2f915c77a95fecf2da25699739a56ee80.
Automaticreview initially rejected sourcetransfer; AWS ownership/tag/IPcheck
confirmed existing Project=ooff worker; review accepted scoped retry. No blocker.

Current exec session6375 runs remote summary/indexedCLItests, then full114889ASO
single native defaultsummary with8threads, noDDG/nohitdump. Driver
benchmarks/summary/run.py records rawoutput/resources/result/Markdown; timeout1800.
Native benchmark results at remotecheckout/benchmarks/summary/iterations/.
Need fetchresults and finalSCN2Atime/outputsize, update docs, stopworker when done.
Existing auto-stop17:47UTC remains. Avoid compiling concurrently with timedsearch.
Local runner gained fallback when/usr/bin/timeabsent; firstfixtureattemptfailed
and retained, correctedfixturepassed. Remoteoriginalrunner/usr/bin/timeavailable.

# Pulp migration — 2026-09-08 15:50 UTC

User requested pulp. Default energy-batch now enables pinned pulp 0.22.3,
stable Rust 1.94, x86-v4 feature. One generic recurrence in src/energy/portable.rs
replaces the AVX-only macro/intrinsic wrappers. NEON4/V3-8/V4-16 automatic dispatch,
scalar tails/fallback. Experimental shared-prefix/min-plus retained off by default.
Global cache and Watt sign unchanged. This does NOT yet implement an integrated
streaming discovery+ddG CLI; that preceding architectural request remains separate.

Pi: 3 alternating matched runs: real unique 20,543 pairs (3 passes) median
1.244699429s vs previous scalar batch 2.643162365s =2.123x. Diverse 1,825 pairs
(10 passes) 1.004052257s vs1.151240257s =1.147x. All energies checked.
See benchmarks/ddg/PULP.md and ledger; timed binaries retained
in data/ddg-pulp/20260908/. No x86 speedup claim: portable gather is currently
per-lane reads (pulp lacks generic gather). Prior intrinsic x86 timings are historical.

Validation: generic ARM release all42tests inclliveVienna2.7 passed; no-default
energytests passed; strictall-targetclippypass; energy module genericx86cross-check
passed via /tmp/oofft-pulp-x86-check (no x86 execution). CI adds scalarfallback
coverage onexistingarchitecturematrix. No remote binary/source overwritten.
Last read-only EC2 check15:42UTC: originalfullwalkPID1333stillrunning,153/225chunks,
~54minuteselapsed. Its binaries unchanged and auto-stop17:47UTC remains scheduled.
Do not attribute wrapper fullwalk timing to the new pulp backend.

# Live monitor checkpoint — full walk continues unchanged

Current fullrun PythonPID1333, worker18.175.151.21 /i-0b5bad3102a5345c5,
m7i.2xlarge32GiB8hardwarethreads. Started~14:48UTC,225chunks*512queries
(lastchunk201),114889total. Latest~79chunks/40448ASOs/146937674intervals.
DO NOTrestart/modifycurrentbenchmark. AllcompletedchunkJSONLarchivesretained.
Output /mnt/oofft-walk/20260908-full-scn2a-paired-direct onnew200GiBvolume
vol-089b5f5f1f8de6a96. Rootremainsvol-0138903fbd36c52bf. Auto-stop17:47UTC.
Timedrun7200sbudget, firstOS-cachewarmupincluded, indexbuildandEC2bootexcluded.

Local read-only monitor RUNNING exec_command session50905:
python3 benchmarks/ddg/watch_full_walk.py; poll withwrite_stdin. Every40sSSH
mirrors result.json anditerations.md to localbenchmarks/ddg/iterations/
20260908-full-scn2a-paired-direct thenrefreshesITERATIONS.md/iterations.csv.
WatcherendsafterfullruncompleteANDViennasampleverificationcomplete.
Postverificationremote oofft-walk-verify-queue.sh waitsPID1333,thenruns
verify_walk_samples.py overfirst16distinctpairs/first4096linesofeverychunk.
ExplicitlySAMPLEverification,noteveryhit. Logfullwalk-verify.log. Helperitself
passed48sites49oraclepairs on3chunkfixture. Needatendfetchoraclemetadata/raw
smallinputs+outputs, retainbinaries/source/hardwareprovenance, updateFULL_WALK.md
withactualtotal+stageaggregate+outputsize, thenstopEC2worker(retainvolumes).

Currentbenchcodefixed: directsingle-lineFASTAintervalreads;15testsallpassARM/x86
inclliveVienna;527956real-siteannotationbytesexactlyequaltooldbackend.
Do notconfusethe28.2945sisolatedvalidationwithmatchedspeedup;23.57swasreference
setup,4.64sprocessing. Globalcachedefault4millionperinvocation,WattddGsign.
No furtherproductioneditsduringtimedrun. Sourcebaselineandinterruptedoldruns
retained. Partialforwardchunkedbaseline7completedchunks;notfullresult.

# Full-walk rerun — 2026-09-08 14:48 UTC

Worker i-0b5bad3102a5345c5 NOW m7i.2xlarge,32GiB/8logical4physicalXeon8488C;
IP18.175.151.21. Rootvolume unchanged. NEW outputvolume
vol-089b5f5f1f8de6a96,200GiB gp3/6000IOPS/500MiB/s mounted/mnt/oofft-walk;
UUIDca62a152-894f-40c7-b532-1542e1c18c7b (notfstab; remountafterreboot).
Mainrun output /mnt/oofft-walk/20260908-full-scn2a-paired-direct;
log /home/ubuntu/ddg-simd-20260908/fullwalk-paired-direct.log.
512querychunks,225chunks,8worker slots,pairedforward/reverseindexes,7200sbudget.
Auto-shutdown17:47UTC. Keepallrawarchivesevidence; stopworkerafterwork.

NEW production improvement: single-line FASTA intervaldirectreads instead of
whole RNA-record loading/normalization for eachbatch. WrappedFASTA/JSONL and
wholetranscript fallback unchanged. All15CLItests inclliveVienna passARMandEC2;
clippypass; full527,956sitechunk outputSHAbyte-identicaltopriorimplementation.
Validation run28.294506sisolated, NOTmatchedspeedup. Artifactremote/localmetadata
benchmarks/ddg/iterations/20260908-direct-interval-full-chunk-validation.
Source sites.rs/testddg_cli.rsupdatedlocal+remote. Baselinebinary retainedremote
baselines/pre-direct-interval/oofft-ddg. Priorchunkedfullrunstoppedafter7complete
chunks; lastoriginalnativechunk6allowedfinish; newdiscoveryjobsstopped, dispatcher
killedafterfreeze toavoidraces. Rootresult.jsonincompletewithreason. Archiveevery
completedchunk preserved. Do not claimfullwalkcompleteduntilcurrentrunfinishes.

# Full-walk live update — 2026-09-08 ~14:32 UTC

Main benchmark NOW walk_chunked.py, PID4037 on18.133.158.117,
/home/ubuntu/ddg-simd-20260908. Output benchmarks/ddg/iterations/
20260908-full-scn2a-forward-chunked; log fullwalk-chunked.log.
114,889 repeat-filtered ASOs,449chunks of256,8worker slots. Eachworker runs
production report(singlethread), nativeDDG(singlethread/default4mcache/Watt),
then archives every report+annotatedJSONL to gzip. Raw-byte SHA256 retained;
only freshly-created disposable uncompressed spools removed AFTERarchive.
Fullquery cache reuse preserved because distinctASOs eachoccur inonechunk.
Reference includes repeats. Forward indexonly11GiB tofit16GiBworker.

Priorunmaskedmonolithic run STOPPED/retained:8reportprocs withforward+reverse
22GiBindexes on16GiBworker,79–83%IOwait, produced~9GBpartialrawreports.
Reason/status diagnostic.json; result.jsonfailed. Neverlabelasfullruntime.
Priorinclude-masked196kstress also stopped after~6m38s (nofullresult).
All14CLItests passedEC2liveVienna currentdefaultcache/Watt. Local198sitefixture
checks monolithic vschunked fulloutput rows equal plusgzrawhashes.
Auto-shutdown17:15UTC. Monitorstorage(current~57GBfree); oldoutputs preserved.
Newrun7200s total budget. No final fullwalkmeasurement yet.

# Active full SCN2A benchmark — 2026-09-08 14:22 UTC

User requested actual full-gene timing; clarified soft-masked design regions.
Recommended/main pool excludes lowercase-overlapping design windows, reference
retains repeats. 114,926 eligible positions /114,889 distinct20mers. Including
all lowercase regions gives197,299positions/196,063distinct20mers, NOT114,889.
Existing full-sequence stress test live on i-0b5bad3102a5345c5,18.133.158.117,
PID1280, 8threads, allsites counts/signatures noCSV/noDDG, started~14:16UTC.
Files remote data/scn2a-full-walk-20260908/discovery.* under ddg-simd-20260908.
Queue fullwalk-queue.sh waits for1280, builds current source, runs live Vienna
CLI tests, then full_walk.py to retain full annotated reports for unmasked
114,889ASOs. Eight report subprocesses (CLI itself singlethread), then merge,
then one native DDG process with8threads/defaultglobalcache/Watt sign.
Runner: benchmarks/ddg/full_walk.py local, remote full_walk.py.
Output remote benchmarks/ddg/iterations/20260908-full-scn2a-unmasked-report-ddg.
Queue log remote fullwalk-queue.log. Auto-shutdown17:15UTC; extend if needed.
68GB disk free before run; watch report/spool storage. No old files deleted.
No end-to-end timing available yet; do not report setup/stress test as full run.

# Runtime estimate correction — 2026-09-08

The earlier “few hours on eight threads” SCN2A estimate is withdrawn as
insufficiently supported. Inspection of the actual retained result confirms
`runs.ooff.output.threads = 1` for the 30.476569099-second 1,000-query full-site
benchmark. It was NOT a measurement using all cores of the larger instance.
Its command also writes every tuple to CSV. Linear scaling to 114,889 queries
would give ~58 minutes at that single-worker throughput, not an eight-thread
runtime; dividing by eight is also unjustified because output locking/I/O and
repetitive-query skew limit scaling.

Separately, the ~3.19-second 100,000-query README result used eight threads in
screen mode (one other-gene witness per ASO), not exhaustive site enumeration.
The ~1.92-second measurement is unique-pair energy computation only for the
1,000-query pilot. None measures the full SCN2A gene walk with all-site ΔΔG.
Full end-to-end latency remains unmeasured; do not repeat a confident hours
estimate or promise seconds based on these different workloads.

# Sign convention update — 2026-09-08

User requested Watt et al. ΔΔG sign. All oofft-ddg paths now calculate
other - intended; output metadata says dg_other - dg_target. Site and standalone/
whole-transcript tests updated (positive, negative, zero, unknown), live Vienna
cache comparisons included. OligoAI verification scripts explicitly negate the
legacy oracle ΔΔG; raw component energies are unchanged. Old benchmark artifacts
retain historical sign. run_real_sites.py now defaults both engines to current
binary so sign/cache changes do not silently mix historical wrappers.

# Latest update — global cache default, 2026-09-08

Global site-energy cache now enabled by default for both engines; configurable
--energy-cache-pairs (4,000,000 default, 0 disabled). Shared across workers and
batches within one invocation, exact full normalized pair keys, caches energies
only. Bounded generations clear on capacity; concurrent misses may duplicate
work. Whole-transcript path unchanged. See benchmarks/ddg/GLOBAL_CACHE.md.
14 CLI tests including live Vienna passed; both energy_vienna tests passed;
targeted clippy passed. No new EC2 timings; worker remains stopped.
User asks OligoAI SCN2A walk latency: provisional few hours on eight threads for
full gene-body walk, prebuilt index, not measured; mature transcript is smaller.
Do not equate 1.92s unique-energy kernel timing with complete user latency.

# Reuse investigation complete — 2026-09-08 13:41 UTC

Userrequestedfurtherinvestigationafter100xdiscussion; no newgoalwascreated.
Previous10xgoalremainscomplete. See benchmarks/ddg/REUSE_INVESTIGATION.md for
self-containedfindings,measurements,reproduction,andnextkernelarchitecture.

Full-reference1000ASOpilot:45,113,535intervals ->2,931,542exact ASO/targetpairs
(15.389xcallreductionpotential),noambiguousintervals. Medianper-ASOreuse1.457x;
15.4xaggregateheavilyrepeat-driven. FullactualVienna2.7.0oraclecoversall2.93m.
Matched8worker directlibrarymedians: native1.92387883s, Vienna63.975902893s,
33.2536xcompute-only; everyenergycheckedineverytimedrun. Nofullreport100xclaim.
NativeglobalproductioncacheNOTimplemented; currentboundedbatchdedupunchanged.

Newenable_shared_paths scheduler regressed~36% fullcorpus(2.6135s vs1.9253s),
~19%sample100k; disabledbydefault. Originalsharedprefixfullcorpusroughlytied.
Exactfrontierworkmodel57,492,634independentcolumns ->17,040,555trienodes,
17,214,096slotswith16lanepadding(3.3399xstatecountreduction). Notanimplemented
energykernelorlatencyclaim. Needslookaheadseparationandboundedancestorgathers.

Changes:thisturnCargoexampleenergy-reuse-census (packedexactkeys,census/expand/
frontier-stats),check_reuse_census.py,actualoracle_pairs.py,experimentalscheduler
src/energy/batch.rs,bench--paths and--baseline-shared,CSVcasecountfield,
liveViennapath-modechecks. Defaultproductionkernelretained. ARMandportablex86
liveoraclepassed; ARM/x86alltargetallfeatureClippypass; censusfixturepasses.
Noactivebenchmark/buildsessions. Noagents/deletions/commit/push/publication.

Rawfullkey/oracledata retainedworkerdata/reuse/full-reference-1000-20260908;
smallcensus/provenance/hasheslocalbenchmarks/ddg/reuse/full-reference-1000-20260908.
Allnewtimedrawstdout/stderr/resultJSONlocalandITERATIONS.md/CSVrefreshed.
Sourcesnapshotsretained data/ddg-baselines/shared-adjacent.
Worker i-0b5bad3102a5345c5 (lastIP18.170.32.22) confirmed STOPPED after completion. Oldcheckpointsbelowhistorical.

# New investigation toward larger DDG gains — 2026-09-08 13:23 UTC

User asked whether100xpossible, then explicitly"continue investigating".
Prior10xgoal remainsCOMPLETE; do notcreateanewgoalwithoutrequest. Noagents.
Workerresumed i-0b5bad3102a5345c5 NEW IP18.170.32.22, autostop14:56:25UTC.
Sameisolatedcheckout /home/ubuntu/ddg-simd-20260908,oldfullrefin/home/ubuntu/ooff.

Exactcensus completedfirstrepetitionof45,113,535 fullreference1000ASOtuples:
2,931,542 unique fullASO/targetpairs,15.389x duplicatefactor,1000distinctASOs,
noNintervals. Ref288419records/2438901225bases. Per-ASOmedianfactor1.4567;
10highest-hitASOs54.26%intervals; globalfactorstronglyrepeat-driven.
Scalarprefixcolumnreuse37,524,536 of57,492,634 (65.27%); 2.879xstate-countfactor,
notruntimegain. Census32.57s includesFASTAloading/CSVscan/sorting/export.
Utilitybenchmarks/ddg/full_reference_reuse.rs exampleenergy-reuse-census.
Exactpackedkeysareu128 little-endian: topbitscanonicalASOid,lower69bits23reversed
basesat3bits/base (A1,C2,G3,U4,N5,zeroendpadding). UnitfixturechecksduplicateASOs,
N,variablelengths,prefixreuseandrepetitionboundary. No hash-collisionapproximation.
Localcensusdata/ddg-reuse/full-reference-1000-20260908; ledgeranalysisrecord
benchmarks/ddg/iterations/20260908T1300-full-reference-sequence-reuse.
Remoteallkeys,queries,samples/oracles:data/reuse/full-reference-1000-20260908.

Newlane-preservingprefixscheduler implementedexperimental enable_shared_paths;
useslongesttargetpaths perASOinSIMDlanes, splitslongpathswithoutpadding.
Existingenable_shared_prefix remainsoriginaladjacentpackexperiment. Bothdisabled
bydefault. Nativecoredefaultunchanged. Newpaths is~19%slower on100kunique sample,
~36%slower onALL2.93m unique pairs (2.6135s vs1.9253s at8threads). Only8.074m
columns reused(~14%). Originalshared onfullcorpus is tied:1.9263 vs1.9257s.
Allenergies independentlycheckedagainstactualViennaRNA2.7.0 onall2.93m pairs.
Oracle-generationwall68.6649s isnotmatchedcompute-onlyspeedcomparison.

Remote session29376 running3matchednative versusdirectViennalibrary measurements
onall2.93munique pairs,8workers,repetitions1. EachViennarunfullwarmup+timedpass,
maytake~2minperinvocation; don'trestartonobservationtimeout. NootherremoteCPUjobs.
Newbenchmarkharnessskipsredundantscalarprechecksforbatchmodes; selectedmodewarmup
and everytimedenergy stillchecked. Oldexperimentsretainedunderdata/ddg-baselines/
shared-adjacent andremote baselines/shared-adjacent.

Localnewfrontier-stats subcommand estimatestrueprefix-trie nodes and SIMDpadding;
notyettransferred/runfull. SeparatinglookaheadfromDP couldreusewholeprefix rather
thanP-1; frontierSIMDcomputeseachprefixnodeonce, butancestor gathers/memorycost
areunproven. Currentmodeisaschedulingworkmodel,notanimplementedenergykernel.
Localbuildsession fromlatesttool needscompletion andcheck_reuse_census.py.
Beforefinish: getdirectViennaresults,runfrontierstatsafterCPUbenchfinishes,
syncallnewrecord/provenance/rawkernels,refreshMD/CSV,writefindings. Finalx86/ARM
ClippyandliveViennatestsneededafterallcurrentedits. Stopworkerwhenfinished.

# Goal completion checkpoint — 2026-09-08

Selected source is validated and measured. Full site annotation target reached:
100000distinctSCN2AASOs/273825SCN1ApremRNAsites,4workers1.106856s versus
Vienna11.809776s =10.67x. At8workers1.001300s versus9.758840s =9.75x,
lowerabsolute native latency. Bothsamecurrentwrapper,3alternatingruns,
allreportfields equal. Kernel28.14x separate; do notconflateitscope.

Final restored nativebinary hash6bcd3a75298c5e13ea07c662c142a078c454d5972df7afdfbc5092fca73979a3
matches selectedbenchmarkartifactexactly. Portablex86CLI12+energy3+minpluspinned1
andactualViennaoracle1passed. ARMCLI12/energy3/minpluspinned1passed.
x86/ARMall-targetall-featureClippypass. Noactivebuild/benchmarksession.
Descriptor-copyexperimentdidnotimproveandwasreverted; rawsourceretained.
Allrecords/profiles syncedlocally; largeannotatedoutputs remainEC2volume.
README/DDGdocs updated, benchmarkITERATIONS.md/CSV refreshed; explanationsin
benchmarks/ddg/OPTIMIZATION_NOTES.md; completionevidenceFINAL_VALIDATION.json.

Worker i-0b5bad3102a5345c5 confirmed STOPPED after completion (lastIP3.9.14.79).
Noagents,deletions,commit,push,publication. Completion audit passed; goal ready to mark complete. Furtherresearchopportunities—fullhumandistinctpaircounting and
cross-SIMDprefixscheduling—are not claimedcompleted. WholetranscriptstillVienna.
Oldnotesbelowarehistorical,includingobsoleteIPs/sessionhandlesandgoals.

# End-to-end target reached at four workers — 2026-09-08 12:27 UTC

Native final-before-copy-experiment at4workers:1.106856s vsVienna11.809776s,
10.66966x,3alternatingruns,100000ASOs/273825sites,allfields equal. Artifact
remote20260908T122341.403813-real-site-annotation. At8workers:1.001300s vs
9.758840s,9.74617x (20260908T122228.749370-real-site-annotation).
The8worker native run is faster in absolute time; do not imply4workers fastest.
Kernel28.14x stillvalid, wholetranscript remainsVienna. No fullhumanDDGtimeclaim.

Code now parallelises complete bounded annotation waves and overlaps nextinput/
previousspoolwrites with scoring. Mainprovenancehash overlapsqueryload when
threads>1. IntendedcachesharedArcMutex withorderedlocking, retainingonceperquery
semantics; lazyworkercaches, immutableArc referenceindex withindependentfilefds.
Normalize consumesownedquerystrings andtransformsbasesinplace. All12CLItests
includingcrossbatchcontrolrows passedx86/ARM. Localenergy/pinnedtests passed.

Finalexperimentusesstd::io::copy withownedduplicateofstdoutfd onUnix toenable
kernel-assistedcopy; sourceinworktree, local12tests pass. Remote session5446
running3matched8worker then4worker runs. First8native~1.02s, noimprovement over
portablebufferedcopy; likelyrevertif4workersdoesnotimprove. Preexperiment sites.rs
retained data/ddg-baselines/parallel-owned-normalize/sites.rs; ooff-ddg.rs unchanged.
Needfinalx86genericCLI/Clippycheckafterdecision, syncnewresultjson/stderr andrefresh
ledger, updateDDGREADMEwithfinalnumbers/reproduction, thenstopworker.
Activeworker3.9.14.79 i-0b5bad3102a5345c5 autostop14:04:27UTC.
Rawlargeannotatedoutputs retainedremoteisolatedcheckout; records/profiles synced
locally via rsync include dirs/result.json/*.stderr. Do notdeleteexperimentdata.
Goalcompletionauditstillrequired; no goalstatusupdateyet.

# Active end-to-end optimisation — 2026-09-08 12:12 UTC

Previous turn was progress: automaticSIMD28x kernel, regressions retained. Goal
still active because end-to-end100kASO annotation measured only5.36x: native
2.251184s versusVienna12.075393s,273825sites,8workers,3runs,allfields match.
Artifact remote20260908T120440.575550-real-site-annotation.

Resumed same verified project worker i-0b5bad3102a5345c5; NEW IP3.9.14.79.
Auto-stop now14:04:27UTCSept8. Same isolated checkout and retainedvolumes.
No agents/deletions/commit/push/publication. Current fullpipelineinputs transferred
at data/ddg-real-sites/20260908T103353.260461-scn2a100000-scn1a onworker.

Implemented bounded whole-batch parallelism for site mode, including JSONparse,
DP andserialization. Each outerworker uses oneinnerCPUthread, totalrequested
coresunchanged. Wholetranscript retains originalinnerparallelism (oneouterworker).
Atmostthreads*batch_size rows buffered; updatedCLIhelp. Workerreferencehandles
openedindependently (File::try_clone wouldshareseekoffset); rangesArcshared.
Workerdesignmaps lazyclones onlyusedqueries. IntendedenergyArcMutex sharedacross
workers; locksacquired sortedqueryID order toavoiddeadlock; vectorbatchcompute
onlyuncachedenergies. Initialversionduplicateintendedcalculation caughtbyexisting
test; correctedandlocal11tests pass. New12thtest invalidcontrolrows acrossbatch
boundaries awaitingremotecheck. Nooutstdoutuntilcompletereportvalidated.
Stageprofiletimings nowcumulativeworkerseconds, explicitlylabelled.

Remote session65543 validates12CLItests, builds, then3matchedsame-current-wrapper
Vienna/nativeiterations. LocalClippy session69026. Readresultsbeforefurtherwork.
Retainedpreparallel sources data/ddg-baselines/pre-pipeline-parallel andremote
binary baselines/pre-pipeline-parallel/oofft-ddg. run_real_sites.py nowaccepts
--threads,--rnaduplex,--platform-label forportableEC2jobs.

# Validation complete — 2026-09-08 12:03 UTC

Generic x86-64 release:11 ddg_cli tests,3 energy tests, min-plus pinned test,
and live ViennaRNA test all passed. Both AVX2/AVX512 tested in portable build.
x86 Clippy initially rejected two redundant -> () store signatures; fixed,
all-target/all-feature Clippy now passed. ARM Clippy passed as well.
No active tool sessions. Current benchmark results are in local ledger.
Stopping worker after this checkpoint to avoid idle compute; retained volume,
source, comparator installs and raw runs remain available for continuation.
Next substantial optimisation is prefix/trie scheduling across SIMD batches;
current shared whole-vector prefix matching misses most scalar reuse. Full
pipeline EC2 timing and full-human distinct sequence-pair counting still pending.
Do not call28x a whole-pipeline or whole-genome result.

# Latest results — 2026-09-08 12:00 UTC

Min-plus x86 pinned/live Vienna tests passed. Unique-pair benchmark regressed:
1.245629 s versus ordinary AVX-512 0.817811 s (1 worker,20543pairs x10,3runs).
Keep disabled; retained iteration20260908T115815.908861-ec2-minplus-avx512-unique.
Direct automaticAVX512 versus Vienna library (8workers,28222pairs x3,3runs):
0.071810 s versus2.020381 s =28.14x kernel only. Iteration
20260908T115853.730209-ec2-auto-avx512-vienna-library-workers-8.
All remote iteration artifacts synced locally and MD/CSV refreshed at11:59.
ARM all-target/all-feature Clippy passed. Generic x86-64 release tests for energy,
energy_vienna,ddg_cli plus liveVienna and x86 all-featureClippy now session31935.
Check completion; this validates runtime dispatch independently of target-cpu=native.
Shared scalar counters667320reused/1214505total columns (~55%); SIMD only5030
reused over10repetitions, so whole-vector prefix constraint blocks most reuse.
Docs/DDG.md now documents exact derivation and successes/regressions.

# DDG automatic SIMD and shared-work checkpoint — 2026-09-08 11:58 UTC

Latest user asks for more ambitious shared partial calculations, automatic AVX-512,
and matrix-style optimisation. No agents or publication authorized. Earlier notes
below are historical. Retain every benchmark attempt, including regressions.

Current EC2 worker remains i-0b5bad3102a5345c5, eu-west-2, 13.135.250.50,
c7i.2xlarge (4 physical cores / 8 threads), auto-stop 13:59:15 UTC Sept8.
Checkout /home/ubuntu/ddg-simd-20260908. ViennaRNA 2.7.0 now installed at
checkout/vienna/bin/RNAduplex, library at vienna/lib/libRNA.a.
AWS read-only describe confirmed project ownership after source-transfer review
initially rejected; retry approved with this evidence. No approval pending.

Implemented default energy-batch feature, runtime AVX-512F+DQ / AVX2 / scalar
selection, one recurrence macro with small width-specific intrinsic wrappers.
Native CLI uses this automatically when --energy-engine rust is selected;
whole-transcript mode still requires Vienna RNAplex. No default backend switch.
Vectorized parameter gathers were essential: initial scalar-gather SIMD regressed,
then AVX2 improved real-site compute about 4.1x over native scalar. Matched direct
Vienna duplexfold comparison at 1/2/4/8 workers showed about 19–21x kernel speedup.
AVX-512 then improved over AVX2 about 1.4x at both 1 and 8 workers.
These are kernel results, not whole-report or full-human timing claims.

Experimental exact shared-prefix scalar DP improved 1.84x over scalar on 20,543
unique real pairs. Combined shared-prefix SIMD was about 4% slower than ordinary
AVX-512, so shared mode is disabled by default. Uses P-1 shared columns because
lookahead dangling/mismatch terms require recomputing the boundary column.

New experiment in src/energy/batch.rs: anti-diagonal min-plus generic loop
recurrence with three sparse range-min tables per retained diagonal. The capped
asymmetry term becomes three exact range minima, removing the inner u/v scan.
Exposed via enable_minplus for benchmark only, disabled by default. Current
remote test/build launched via SSH session91801; inspect result before timing.
Local ARM pinned test passed (scalar fallback); does not validate x86 intrinsics.
Benchmark mode --minplus added. Live Vienna test covers both SIMD widths and
shared reuse; min-plus pinned test forces each rare length through full lanes.

Next: finish x86 correctness, benchmark min-plus versus ordinary AVX-512 on unique
real pairs, sync all remote iterations, refresh ITERATIONS.md/CSV. Validate generic
x86 build runtime dispatch and current CLI tests/Clippy. Full pipeline SIMD
measurement remains outstanding; earlier strongest matched Pi pipeline was4.45x.
Full-reference unique ASO/target pair count remains unmeasured (45m intervals are
not 45m independent energy computations). Do not invent a deduplicated estimate.

---

# EC2 DDG SIMD/core-scaling focus — 2026-09-08 11:02 UTC

User revised active goal to 10× DDG versus Vienna, then explicitly steered toward
better vectorisation/core parallelisation and testing on EC2. Prior turn PROGRESS.
Do not continue treating1000× as the active objective. Focus now kernel batching,
SIMD and core scaling rather than further JSON-only tuning.

Resumed retained EC2 i-0b5bad3102a5345c5, eu-west-2, IP13.135.250.50.
Instance c7i.2xlarge: Xeon Platinum8488C,4physicalcores/8hardwarethreads,
AVX2 and AVX512 available. CPU0..3 separatecores,4..7 siblingthreads.
SSH key data/ec2/controller.key, known_hosts data/ec2/known_hosts.
Auto-stop scheduled13:59:15UTCSept8 (3hours); stop earlier if finished.
Isolated checkout /home/ubuntu/ddg-simd-20260908, old human data untouched.
Current source tar /tmp/oofft-ddg-ec2-source.tgz transferred/extracted there.
Remote cargo ~/.cargo/bin/cargo; release oofft-ddg+energy-bench build PASSED,
build.log/build.exit0. RNAduplex not installed in standard/localbin paths yet.
Must install pinned Vienna2.7.0 and measure matched resources before claiming10×.

Extended kernel_bench.rs with optional threads argument (arg3, default1), one
workspace per worker and unchanged checksum/oracle checks. run_kernel.py accepts
--threads. Remote firstsweep1/2/4/8workers,100reps×1825cases,3measurements each,
completed via session55825. Median seconds:1worker6.693740,2workers3.558603,
4workers1.831031,8workers1.471562; every checksum/golden check passed.
4workers3.66× versus1worker;8threads4.55× versus1worker (1.24× over4). Results remote benchmarks/ddg/iterations/*ec2-initial-workers-*.
Results copied locally and ledger refreshed, including EC2 platform metadata.
Worker remains running under the existing automatic stop for next SIMD/Vienna work. Session72453 was the SSH
launch wrapper for the now-finished build; authoritative remote build.exit=0.

Immediately preceding local work enabled sha2 asm feature (runtime dispatch with
software fallback); new Cargo.lock dependencysha2-asm0.6.4. All273825 report rows
matched previous native3reps:6.352475s vs7.203925s (1.134×). Artifact
20260908T105315.328753-real-site-annotation. Baseline data/ddg-baselines/software-sha.
Also measured same CURRENT executable Rust vsVienna, removing historical wrapper
advantage: Rust6.322299s/Vienna28.108133s =4.44587×, allrows equal3reps.
Artifact20260908T105426.272867-real-site-annotation. This is the strongest matched
pipeline comparator so far, not10×. Latest SHA change still needs full local
checks; new kernel parallel harness needs scaling/output audit after remote runs.
No agents, deletions, commit, push or publication. Earlier notes historical.

---

# Lazy intended scoring and JSON fast path — 2026-09-08 10:47 UTC

Previous turn PROGRESS; goal ACTIVE,1000× unproven. Implemented intended energies
on first reported site, cached per query across batches; all query validation
remains eager. 100K-ASO/273825-site matched-native3reps: lazy median11.930965s
vs eager12.454225s, 1.04386×. Artifact20260908T104143.986110-real-site-annotation.
Saved baseline data/ddg-baselines/eager-intended/oofft-ddg.

Then added minimal ReportFields parsing and original-site JSON passthrough:
parse only type/queryID/recordID/start/end; keep original line for output;
append computed metadata with reusable serialization buffer. Root ddg or
energy_annotation present (even null) triggers complete Value parsing; existing
supplied_ddg and rejection semantics preserved. Envelope deserialization errors
fall back to original full parser. Other rows still use full parsing.
11DDG CLI tests passed, including unused malformed query, cross-batch intended
cache, arbitrary nested fields, escaping/braces/Unicode/whitespace/null DDG.

Matched-native100K3reps: JSONfast median7.128468s vs priorlazy11.999032s,
1.68326×; all273825 rows equal. Artifact20260908T104527.038836-real-site-annotation.
Baseline saved data/ddg-baselines/lazy-intended/oofft-ddg.
Direct originalVienna comparison3reps completed, all273825 rows equal:
Rust median7.180837s vsVienna38.842693s =5.40922×. Artifact
20260908T104655.104800-real-site-annotation. Session10712 terminal.
Native pairs296647/unique238481: lazy scoring avoids77178intended calculations
(corrects the earlier arithmetic typo77778). Profile duplex~3.10s, parse~1.00s,
serialize~0.78s, annotation batches~3.64s (overlaps duplex). New primary bottleneck
is again kernel work, plus remaining IO/validation rather than JSON object trees.
Full cargo test --locked --all-targets and all-target/all-feature Clippy passed.
Log /tmp/oofft-ddg-tests-20260908-json-fast.log. 11DDG CLI tests plus1825duplex
and11.16Mloop oracle checks passed. Final10K-ASO3reps: all28222sites equal,
Rust0.728725s vsVienna4.181658s =5.738×; artifact
20260908T105047.125660-real-site-annotation. Synthetic10Ksites/1000records0.202s,
all direct oracle energies equal, artifact20260908T105105.109056-estimate-sites-10000-records1000-rust.
All tool sessions terminal. Ledger/docs refreshed; no 1000× claim. Release executable built with both experiments disabled.
No agents, deletions, EC2 starts, commits or publication. Earlier notes historical.

---

# Batch reuse and 100K-ASO calibration — 2026-09-08 10:36 UTC

Previous goal turn PROGRESS; target1000× remains ACTIVE/unproven.
Native duplex stages now deduplicate full borrowed ASO/target-sequence keys within
each bounded batch, map energies back to every original row, and report native_pairs
and native_unique_pairs in profile. No cross-batch/unbounded cache. Nine DDG CLI tests
pass, including different coordinates/lengths/ASOs. Seven alternating matched-native
real1922-site runs improved median ~2% (0.09775 vs0.09977s), all fields equal.
Baseline saved data/ddg-baselines/pre-batch-reuse/oofft-ddg.

Expanded actual SCN2A distinct-ASO→SCN1A pre-mRNA workload:
10000ASOs:28222sites,20543unique energy pairs; discovery6.77s.
100000ASOs:273825sites,201640unique energy pairs; discovery67.40s.
Input directories data/ddg-real-sites/20260908T103345.681632-scn2a10000-scn1a
and data/ddg-real-sites/20260908T103353.260461-scn2a100000-scn1a.
These are transcript-coordinate workloads, NOT human-genome timing claims.
10K annotation3reps: Rust median1.290205s vsVienna4.200932s =3.256×,
all28222 annotated rows equal. Artifact20260908T103534.925438-real-site-annotation.
100K annotation comparison completed: all273825 rows equal across3reps,
Rust median12.744874s vsVienna38.922903s =3.054×. Artifact:
20260908T103554.623075-real-site-annotation. All tool sessions terminal.
Native profile:373825pairs,315659unique (58166 reused), duplex~3.90s,
report parse~3.48s, serialization~2.59s; output360306267bytes. Annotation batch
~3.60s overlaps off-target duplex work; do not sum overlapping scopes.
Only22822of100000ASOs have reported sites. NEXT: defer intended energy calculation
until a query first has a site, retaining all input validation and every output.
Current pipeline eagerly calculates intended energies for all100000 queries.
Could avoid77178unneeded calculations without changing annotation semantics.
Then address JSON parsing/serialization, now comparable to kernel cost.
Nine DDG CLI tests, all-target/all-feature Clippy, Python syntax checks passed.
Default release rebuilt and used for these measurements; experiments disabled.

run_real_sites.py now streams stdout to files and compares parsed rows in bounded
memory after timing; stdout_sink metadata records the measurement change. Supports
--baseline-engine rust for matched native comparisons. New attempts retain profiles;
CSV includes ASO/site counts and native total/unique pair counts when available.
No agents, deletions, EC2 starts, commit or publication. Earlier notes historical.

---

# Rust DDG optimisation checkpoint — 2026-09-08 10:30 UTC

Previous goal turn made PROGRESS. Goal ACTIVE, 1000× remains unproven.
This turn explicitly unrolled nine small-loop cases; diverse golden kernel median
1.16565s vs portable reused-workspace1.18012s (~1% gain). Real1922-site run
0.09929s vs retainedVienna0.26504s (2.67×), every field equal.

Implemented an exact relaxed-matching pruning bound as disabled feature
energy-prune. Prefix/suffix maximum pairing counts times global minimum loop cost
bound all paths through a cell; exact uninterrupted helices provide upper bounds.
All1825 oracle energies and all1922 real report rows match, but median kernel
1.38431s vs unrolled1.14570s: ~21% regression. Real native0.10315s; synthetic
10000sites0.237s. Do not enable by default. Proof sketch in src/energy/README.md.
Binaries retained data/ddg-baselines/{unrolled-small-loops,relaxed-matching-prune}.
The pre-unrolled-small-loops binary copied before rebuild was the old NEON build;
README there records that provenance. It was NOT used as the portable comparator.

Ledger iterations: 20260908T102704.663301-unrolled-small-loops-golden,
20260908T102914.989662-relaxed-matching-prune-golden and adjacent end-to-end runs.
No agents, deletions, EC2 starts, commits or publication. All-feature library, energy and DDG CLI tests passed, including both experimental
features together; all-target/all-feature Clippy passed. Default release binaries
were rebuilt with both experimental features disabled. Formatting/diff checks pass.
No live tool sessions remain. Earlier checkpoints are historical.

---

# Rust DDG checkpoint — 2026-09-08 10:25 UTC

Native site-energy implementation is complete locally; the broader 1000× speed
goal remains ACTIVE and unproven. All benchmark attempts and resources retained.
No commit, push, publication or EC2 start. Retained worker i-0b5bad3102a5345c5
confirmed STOPPED this turn; data/ec2/current.json is stale.

Added reusable DuplexWorkspace per worker, exact forward/reverse reuse oracle
checks, optional energy-neon reduction, and wall-time profiling. Workspace-only
compute gain ~0.4%; NEON ~3.6% regression, so NEON stays disabled by default.
Factored Rust recurrence remains independent of Vienna calls/linking; empirical
parameter data retain original attribution and distribution review remains open.

Real SCN2A1000 ASOs → SCN1A transcript discovery (uncapped k3) found 1922 sites,
1604 distinct query/target-sequence pairs. Local input directory:
data/ddg-real-sites/20260908T101024.131957-scn2a1000-scn1a.
Only ~17% potential identical-pair reuse; do not assume extreme deduplication.
Three initial real annotation runs: 2.57× old Vienna pipeline, every row equal.
Profiling put duplex stages around37–39% of runtime on measured workloads.

Latest change serializes typed energy metadata alongside original report Values,
removes per-record work directories in site mode, preserves previous supplied_ddg
and all output fields. Latest alternating three-run real benchmark:
benchmarks/ddg/iterations/20260908T102429.623503-real-site-annotation/result.json
Native median0.098448661s vs retainedVienna0.268975852s, 2.732×, all1922 rows equal.
Latest synthetic10000sites/1000records native0.203s, all energies match:
benchmarks/ddg/iterations/20260908T102431.200096-estimate-sites-10000-records1000-rust.
These are Pi four-worker site-annotation results, not genome-wide estimates.

Full cargo test --locked --all-targets passed after these changes, including
1825 duplexes and 11,160,000 loop cases. Log /tmp/oofft-ddg-all-tests-20260908.log.
Ledger now distinguishes baseline names and compute-vs-whole-process scopes.
New reproducible helpers: prepare_real_sites.py, run_real_sites.py, run_kernel.py.
Potential next work: stronger exact bounds, SIMD across pairs, avoiding full JSON
Value parsing, bounded exact pair reuse on measured real workloads. Keep default
Vienna until rollout decision; Rust remains --energy-engine rust for --sites.
Whole-transcript mode still uses actual RNAplex semantics and rejects Rust.

Earlier checkpoints below are historical.

---

# ACTIVE GOAL: independent Rust ΔΔG, target 1000× — 2026-09-08

User explicitly requests implementation from scratch in Rust, with exactly
matching outputs. Goal is ACTIVE and 1000× has NOT been demonstrated. Do not
mark complete or fall back to an orchestration-only deliverable. No agents.
No deletions. All benchmark attempts retained. Previous goal turn: PROGRESS.

Implemented independent native site-energy DP in src/energy/mod.rs, plus a
factored recurrence in fast.rs. No ViennaRNA calls/linking in Rust energy
computation. Empirical default 37 C parameters exported from pinned installed
ViennaRNA into src/energy/turner2004_37c.json; provenance and upstream data notice
retained. Data-distribution terms need review before any package publication.
Current --energy-engine rust enables it for --sites; default remains Vienna
while validation/optimization proceeds. Whole-transcript mode remains the
separate existing RNAplex path; Rust+whole-transcript explicitly rejected.

1825 complete duplexes match pinned RNAduplex energies exactly, including
random/unrelated pairs of lengths1–45, near-complementary pairs, N, homopolymers
and no-pair sentinel100000 kcal/mol. Additional C oracle enumerates all 496
loop shapes (total<=30), six canonical/GU pair types on both sides, and all
5^4 adjacent-base combinations: 11,160,000 loop energies, hashed by shape for
Rust-only CI. All-target test session84321 passed, including the full 11.16M energy oracle.
Session97305 also passed current Clippy, the exhaustive lower-bound assertion,
and release rebuild. No running tool sessions remain from this goal turn.

Native scalar recurrence retained as duplex_energy_scalar. Fast recurrence
factors generic mismatch terms into predecessor DP values, separates small
loops and bulges, and uses exact integer costs. Admissible loop/row lower
bounds derived from parameter minima. No energy approximation or edit cutoff.
Cross-record batching also removes one process launch group per RNA record.

Measurements on Pi, four workers, 1000 synthetic ASOs and 10000 near-match sites:
old Vienna pipeline scattered1000records5.554s; first scalarRust0.589s;
prunedRust0.342s; factoredRust0.348s (noise-level difference). Groupedone-record
prunedRust0.271s versusold0.830s. All component energies and report rows matched.
This is ~16× on scattered records, NOT1000× and not a measured genome-wide run.
Compute-only golden1825pairs×10, one CPU: prunedscalar1.415999s versus
factored1.191490s. Separate kernel/whole-process scopes retained in CSV.
Baseline binaries retained data/ddg-baselines/pre-cross-record and pruned-scalar.

Next: poll/finish current tests, recheck Clippy; add/run strict bound assertions.
Profile actual kernel overhead, improve/vectorize independent pair batches,
consider reuse of overlapping intervals and identical ASO/target sequences on
real references without assuming duplication. Compare equally resourced,
identical workloads with full output equality; don't count changing transcript
scan to local scoring as a speedup. Keep broader golden/random/adversarial
coverage as optimization changes. Benchmark scripts: estimate_sites.py
--energy-engine rust, kernel_bench.rs example, generate_energy_golden.py,
generate_loop_golden.py. Rebuild release before timing; don't overlap timings
with compilation. Refresh benchmarks/ddg/ITERATIONS.md and iterations.csv.

Project rename/site annotation work remains local/uncommitted. No GitHub auth
available, repo URL remains barneyhill/ooff. No EC2 workers started in this goal.
Rust env: CARGO_HOME=/tmp/ooff-cargo RUSTUP_HOME=/tmp/ooff-rustup,
/tmp/ooff-cargo/bin/cargo. Native build target/release/oofft-ddg.
Earlier notes below are historical.

---

# oofft rename and site ΔΔG annotations — 2026-09-08

User confirmed spelling **oofft**. Cargo package, public binaries, Rust imports,
CI/release archive names, README commands, current benchmark entry points and
plot legend updated. Source filenames and historical format/measurement IDs
remain compatible. Workspace and GitHub URL remain /home/barneyh/ooff and
barneyhill/ooff; no GitHub API credentials/CLI available here. No push or publish.

`oofft-ddg --sites REPORT --queries ORIGINAL --reference ORIGINAL` annotates all
reported intervals with RNAduplex intended minus RNAduplex interval energy.
Intended energy computed once per query; optional query target field, otherwise
explicitly labelled perfect complement. No fixed-CIGAR thermodynamic constraint,
no flanks, no hit filtering, no exact shortcut for site mode. RNA:RNA defaults.
Preserves original coordinates, site identities, summaries and capped status.
Old supplied ddg preserved separately. Bounded row batches and reference byte
ranges, retained spool, SHA256 input checks and no stdout on scoring/input error.
`--whole-transcript` retains the actual OligoAI RNAplex whole-record calculation,
including the exact-match shortcut and separate best-position annotation.

All Rust tests and Clippy pass, workflow syntax checked, source package verified
offline, ARM64 release archive smoke-tested. Real engines verified 20 discovered
intervals of lengths17–23 against direct RNAduplex, and whole-record annotations
against actual OligoAI for every site. Older 3.90x/29.75x performance measurements
remain explicitly whole-transcript benchmarks, not site-scoring speed claims.
Every iteration is retained in benchmarks/ddg/ITERATIONS.md and iterations.csv.
Details and commands: docs/DDG.md. Changes remain uncommitted and unpushed.
Earlier notes below are historical.

---

# ΔΔG implementation validated — 2026-09-08 UTC

New optional `ooff-ddg` command preserves OligoAI v2's RNAduplex/RNAplex
calculation, sign and exact-match shortcut. No new thermodynamic recurrence.
Native query indexing, deduplication and parallel processes measured on this Pi:
3.90× for 128 distinct designs, 29.75× for 16 designs repeated eight times,
9.17× for 1,000 exact matches (shortcut only). Three repetitions of each real
workload matched every field against the actual OligoAI TypeScript function.
Final binary also passed 16/18/20mer insertion/deletion fixtures at 1/4 workers.
All attempts, including the initial parser failure, retained under benchmarks/ddg;
ITERATIONS.md and iterations.csv provide plot-ready timings and source hashes.

Full Rust tests and Clippy passed. CI includes the tests and release packaging
includes ooff-ddg; ViennaRNA remains an external optional runtime prerequisite.
No OligoAI source edits, no EC2 launches and no release publication in this work.
Scientific assumptions and reproduction: docs/DDG.md. Offline cargo package
verification and ARM64 release archive smoke tests passed. Online packaging was
stopped after DNS failures; all cached dependencies verified offline.
Implementation and evidence are local, not yet committed or pushed.
Earlier notes below are historical.

---

# Follow-up completed — 2026-09-07 23:42 UTC / September 8 London

Sassy eight-CPU screening series complete:15measurements, allqueriesrecovered.
100kmedian19.323s vsnative3.191373s =6.05x; explicitlydistinguishedfromoriginal
one-threadfull-output10.67x. Newgraphsearchtimeonly, fivecurves, no triangles,
indexbuildsin caption. README/methods/CSV/Markdown/evidence updated;25cells audited.
SourceworkerSTOPPEDagainverified; allfourdedicatedworkersstoppedandvolumesretained.
FinalinventorywithSassy6976files/105620185074bytes localdata/ec2. No raw deletion.
Newwork ready to commit/push and verify hosted CI. No pendingEC2jobs.
PreviousGitHubCIallfourplatformsgreen; publishing waitsforversiontag, nonepushed.
Earlier active notes below are history.

---

# Active follow-up — 2026-09-07 23:37 UTC / September 8 London

USER approved search-only plot, no triangles, and matched eight-CPU Sassy line.
Original benchmark goal was completed; this is newly requested follow-up work.
GitHub CI/release setup committed/pushed; all four hosted platforms green at
d635f46. Latest main c0882d1 changed plot to build+search; now reverting graph
metric per user (search only, builds in caption, timeout text). Worktree edits
not yet committed. No crates release tag created or package published.

Resumed source/native i-0b5bad3102a5345c5 c7i.2xlarge8vCPU16GiB,
new IP3.8.120.217. Other workers remain stopped; no new instances or disks.
Auto-stop02:36UTCSept8. External comparator path
/home/ubuntu/comparators/sassy-classic-screen-0.2.6/target/release/sassy-classic-screen.
SHA c8458fa0131e8249a541bc56995434edf5eeb8a62f23a6279e4ac047118ba394.
New source benchmarks/classic/sassy_screen.rs, eight outer query workers share
same eligible reference; Sassy0.2.6 IUPAC batched forward API, no nestedthreadpool.
Independently scalarchecked witness intervals emitted4colTSV (existingblastreader).
No persistent index. Native dependency graph unchanged; external adapter uses
native IntervalDistance only for endpoint-to-interval reconstruction, then Python
scalar verifier independently checks every human witness.

Build/test/pilot retained in classiciterations. Scalaroracle k0..3 at1/8workers
ALLPASS; human100pilot18.584675s/100witnesses/zero rejected. Measurement series
launched23:36, --counts10 100 1000 10000 100000 --repetitions3 --timeout600
--run-timeout600. Logresults/sassy-classic-series.log; completioncodefile
results/sassy-classic-series.exit. Active SSH tool session12634 may remainopen
because background shell inheritedconnection; DO NOTrestart job. Inspectprocess.
ExternalCargo.lock/TOMLcopied benchmarks/classic/sassy-Cargo.{lock,toml}.
Buildscript nowusespinnedlock forfuturefreshreproduction; don'tcopychangedhelpers
toactiveworker untilmeasurementends (provenance).
Need collect measurements, final5tool×5sizeaudit, rendersearch-onlysingleplot,
README/methodsupdate, recordalliterations, finalinventory/stopverifyworker,
commit/pushandmonitorCI. Earlier final states below are history.

---

# Final state — 2026-09-07 23:08 UTC / September 8 Europe/London

COMPLETE: original 10× Sassy objective and all added benchmark deliverables.
Full output median10.674161x, whole-process10.573411x, literal135,340,605tuples
exactlyequal/unique across3reps. Sassy external EC2comparatoronly.
Powers-of-ten README plot10/100/1000/10000/100000, explicitlog10; buildtimes/RAM
separate. All20tool/sizeoutcomes audited. 100k native3.191373smedian (first51.6648s
retained), BWA246.898375smedian/98.774%recovery, BLAST600stimeout, minimap2OOM32GiB
at421.161s. KernelPID5247confirmed; resource_failure.json retained; notatimeout.
Usercancelled26Kremainingreps; oldmeasurementsandcancelledrep2retained.
AllfourdedicatedworkersSTOPPEDverified23:06, alleightvolumesretained.
Finalminimapinventory290files/78,843,699,372bytes localdata/ec2; allmetadata synced.
FinalPNGvisuallychecked; plots/CSV/Markdown containactualoutcomesandlimitations.
Completion evidence benchmarks/completion-evidence.json; detailedrequirements
in docs/VERIFICATION.md; finalclassiccoverage benchmarks/classic/series-audit.json.
No commit/pushrequested orperformed. No remaininglivejobs orrequiredwork.
Earlier updates below are historical and superseded.

---

# Completion audit progress — 2026-09-07 22:59 UTC

Previous goal turn PROGRESS: powers-of-ten scope implemented and old26Kcancelled.
Current turn PROGRESS: verified exact135,340,605tuple result and recomputed10.674161x
median/10.573411x whole-process speedups; one search thread bothengines. Saved
benchmarks/completion-evidence.json. Cargo.lock excludesSassy. PlotCSV hasonly
10/100/1000/10000/100000; native+BWA3completeeach. Fixed staleREADME/HANDOFF/reference
and artifact state prose. BWA STOPPEDverified22:57; native+BLASTalsoSTOPPED.
Minimap100kPID5247 confirmedLIVE22:57:29 at2:15, parent4646. Extra65GiBfree.
No work restart. Deadline~23:05:14UTC. Final inventoryhelper and batchselection
sidecar copied toworkerwithoutchangingactivebenchmarkhelpers. Need final100k,
rsync-Lmetadata+seriesmarker,finalinventory,stopMM/verifyretention,plot+auditfinish.

---

# Latest steering — 2026-09-07 22:56 UTC

USER: "dont do 26K - OUT OF PLACE, i like the log 10 ones".
README plot, scale defaults and completion coverage now ONLY powers of ten:
10/100/1000/10000/100000. All historical intermediate measurements retained.
Cancelled minimap2 26643 rep2 verifier PGID4958 with TERM; runner4013 wrote
failure record and exited, so no rep3. Decision benchmarks/classic/batch-selection.json.
Queued 100000 runner now PID4646; searchPID5247 / time5246, confirmedLIVE22:55:31.
Artifact20260907T225513.797152-measurement-minimap2-100000-r1-search.
New total600 budget should terminate by23:05:14 if still searching. Do not restart.
Need collect cancellation + final100k, inventory/stop minimap, verify BWAstopped,
final plots/docs/audit. Prior26Kremaining requirements superseded by user.

---

# Live update — 2026-09-07 22:54 UTC

Current goal turn: PROGRESS. Collected and audited final BWA 100,000 runs:
282.756636745 / 246.455577901 / 246.898375245 seconds, 98,774 recovered each.
BLAST 100,000 terminated at 600.016508332 seconds with timeout=true, return -15.
All metadata local, Markdown/CSV refreshed and log10 figure regenerated.
Audit now has ONLY minimap2 26,643 and 100,000 pending.
Native and BLAST STOPPED, verified 22:53 UTC. BWA stop requested 22:54; verify.
Final local inventories: data/ec2/artifact-inventory-native-100000.json (6,104
files / 105,333,552,190 bytes); artifact-inventory-blast-100000.json (195 files /
3,174,561,805 bytes); artifact-inventory-bwa-100000.json (440 / 9,743,343,829).

Minimap2 i-0b880ceee50e0c3fa, 18.130.251.80: parent4013 confirmed live22:51.
26,643 rep1 total1018.81427395s, 25,359 recovered; rep2 search456.591988352s
completed and verification running. Rep3 follows; queued shell4646 confirmed
live, starts 100,000 with600total budget after parent4013 exits. Extra output
volume had65GiB free22:51. Do not restart; collect with rsync -L.
Estimated remaining ~30minutes. Auto-stop ~00:33 Sept8.

User gene/location question answered and actual report fixture verified:
10 annotated site rows plus run_complete. README now names report besidegraph.
Methods, verification and artifact docs updated for eight sizes and new state.
Need finish minimap2, final collection/inventory/stop, final plot inspection,
completion audit and goal completion. No blockers. Earlier updates below are history.

---

Native100kCOMPLETE:51.664824259/3.191373288/3.089922757seconds,100000/100000verifiedeach. Allthree retainedincludingfirstafterrestart; localmetadata+log10plotupdated. Do notclaimcoldtimingsuniversally3sec. Worker35.177.75.113stillrunningpendingnewfinalinventory/stop. Publishinputsnowincludeslarge-batchmanifest+origins; isolated_job exposesOOFF_RUN_TIMEOUT_SECONDSdefault600.

# Live update — 2026-09-07 22:41 UTC

NEW USER SCOPE: add configurable timeout and extend to100000ASOs. Implemented
scale.py --run-timeout600 complete-run budget acrosssearch/conversion/verification,
--timeout remainsperstagecap. Bothrecordedmetadata; defaults600,positivevalidated.
Oldrunsretainperstage600semantics. Real2subprocess EC2test_timeout.py PASSED
(secondstagecancelledafterfirstconsumedbudget). Artifactsphase timeout_test.

Prepared EXACT100000unique20nt queries preservingexisting26643prefix, plus73357
SCN2Agene-bodyreference20mers orderedbySHA256(target), deduplicated. Generator
benchmarks/classic/add_large_batch.py, source197318nt SCN2A-reference-v1.fa has
114889uniquevalid20mers. Manifest data/classic-v1/large-batch-manifest.json plus
originsCSV. ReferenceSHA a79b46c86cb9e292d5a3f6ca2e34b8f2e8e2d8579c5efa1dfedaccdf97ed8e9f.
ASOFASTA SHA74623c7a730efb54aeb0a83c5c53e5101e86b088b07d6775912ec57587c9365e;
targetFASTA5478a641448681d573569957dfc627e85a205c41f8143086e9bb3b8e85c0d5f4.
ClearlyMIXED allele/referencequeries, not100kobservedvariants. Asyncsourcechoice
asked; noanswerafter>60s, statedassumptionandproceeded. Newlog10plotsretainprefix.

Restartedoriginalnative/BWA/BLAST8vCPU16GiB. NEWpublicIPs:
native35.177.75.113 /i-0b5bad3102a5345c5, PID1440 run_large_batch.py --toolooff
BWA3.8.78.225 /i-0eed25aed0ad7dd2c, PID1351 --toolbwa
BLAST13.40.47.32 /i-043c11d9e3597df11, PID1355 --toolblast
Each100000x3/total600s timeout, firsttimeoutskiprest. Logsresults/TOOL-100000.log.
run_large_batch.py verifieseveryinputSHA, uniqueness, RCandretainedprefixbefore
measurement; writesresults/TOOL-100000-series.json aftercompleted/timeoutseries.
Allnewworkersshutdownresetandverified~00:37Sept8; noresource/rebuildchanges.

Minimapstill18.130.251.80/i-0b880ceee50e0c3fa. Existing26643retryparent4013 live;
firstsearchFINISHED455.338s (retainedlargeSAM), verificationnowrunning. Old600per
stage semantics unchanged. NEW100kqueued shell waits4013 beforecopying staged
scale/command/run_large fromdata/large-batch-stage intoactivehelpers, thenruns
100kwithOOFF_CLASSIC_STORAGE extra100GiB andtotal600budget. Queue shell PID4646.
Logresults/minimap2-100000-queued.log. Do notstartoverlappingtimingjobs.

Need update README/methods/auditplanned8sizes andmixedqueryprovenance,totaltimeout,
allnewresults/plotdata andnewhardwarestate. Audithelperalreadyexpects100000.
Needcollectnewtests/results,finishMMold+100k/newthreeworker100k, finalinventories,
stopallfourworkersagainandverify. Goalremainsactive; user100kworknotcomplete.

---

# Live update — 2026-09-07 22:29 UTC

PROGRESS: BLAST series terminal, PIDs2500/3982 missing. Audited13measurements:
10/30/100 each3complete;1000/5000/10000/26643 eachone600stimeout. Auditlocal
 data/ec2/blast-final-series-audit.json; inventory182files3157317456bytes local
 data/ec2/artifact-inventory-classic-blast.json. Allfinalmetadata synced.
Stoprequested i-043c11d9e3597df11, verify stopped. README/methods nowBLASTfinal,
onlyminimaplargeststillrunning. Native+BWAstoppedalready. NoSpotcancellations.
Minimapretryparent4013/engine4016 confirmedlive22:27, artifact222625.844975-r1,
extraoutputvolumehas93GiBspaceinitial. Needfinishthatseries, rsync-Lmetadata,
finalinventory(includesdata/classic-extra), finalplots/docs/audit,stopworker,
verifyvolumesretained andgoalcompletion. CurrentturnPROGRESS; no blockers.

---

# Live update — 2026-09-07 22:26 UTC

PROGRESS: minimap26643r1 process TERMINATED with ENOSPC before onlinefilesystem
growthfinished. OriginalpartialSAM11947634688bytes and0-byte result/resources
retained. Recoveredresult haswall_seconds=None,returncode=None (unknown),explicit
failure; emptyresult renamedresult.partial.json. command.refresh handlesmissing
timing asunavailable; no inventedelapsedtime/no screeningpointforfailedattempt.
Addednewblank100GiB encryptedgp3volumevol-0cb196a72cc0da8de, attached/formatted
verifiedserialnvme1n1;mounted /home/ubuntu/ooff/data/classic-extra, UUID
139cca5c-9449-4df4-9fd1-7d982da97521,93GiBfree. Rootalso100GiBnow.
NewOOFF_CLASSIC_STORAGE supportsnewiterationdirs onextradisk withsymlinksfrom
standarditerationspath. Remote additional-storage-checkpassed. Localcheckcould
notexecute duecontrollerlacking/usr/bin/time; no benchmarktimingwas claimed.

NEWminimaplargest retry launched22:26: results/minimap2-largest-storage-retry1.log,
--counts26643 --repetitions3 --timeout600,8threads,samebinary/options. Retry runner PID4013 (verify withcurrentps). Collectmetadata usingrsync -azL --exclude stdout (MUST-L
forsymlinkdirs). Inventoryscriptnowincludesdata/classic-extra. Docsreprolaunch
minimapinitialdisk200GiB; ARTIFACTS describes bothvolumes andmanualremountUUID.
BLASTfinalPID3982stillrunning~22:25, expected600stimeout22:28:24. Stopaftercopying
finalmetadata/inventory, retaininstance/request/disk. Native+BWAalreadySTOPPED.
No goalcompletionuntilremainingretryandfinalauditdone.

---

Minimap disk expanded50→100GiB22:20 (same3000IOPS/125MiB/s), growpart+resize2fs verified complete (session82533 exit0); filesystem96GiB with49GiB free. Interventionrecord benchmarks/classic/minimap2-storage-intervention.json; affected26643r1search retained.

# Live update — 2026-09-07 22:20 UTC

User asked whether ooff has an index and requested log10 graph. Answered yes:
reusable11.6GB FMindex, measured206s8threadbuild, separatelyshown. Both timingaxes
andbuildvalueaxes now EXPLICIT base=10 inplotcode, regeneratedSVG/PNG/CSV.
Recovery percentage remainslinear. Prior goalturn PROGRESS: audited original
10.674161x numerical evidence and fullrowCountercomparison, corrected stale
ALGORITHM.md queuedwording. Current PROGRESS: collectedBLAST10000timeout and
updatedplot; verifiedBLASTfinal26643 PID3982 since22:18:23, minimap26643r1
PID3053 since22:14:44. Bothlive; do notrestart. Minimaproot37GBused/12GBfree,
RAM~25GBused; inspectingstdoutsize toensurefinalrepsfit. BLASTroot43GBfree.
Stillwaitbothfinalseries, thencollectinventories/metadata/refreshfinaldocs/stop.

---

# Live update — 2026-09-07 22:16 UTC

Previous goal turn was PROGRESS: two experiments, witness fix, measured README
figures and native/BWA shutdown. Current turn PROGRESS plus VERIFIED WAIT:
BLAST live PID3468 (10000ASO search), parent2500; minimap parent2171 and10000r3
verification2817 confirmed live22:14. Do not restart. Remaining26643 next.
Collected latestmetadata and added explicit search_seconds, conversion_seconds,
verification_seconds (process wall), tool_seconds columns to scaling plot CSV.
Main figure remains total verified wall time. Docs explain audit overhead.
Re-render after each finalcollection. Native/BWA stoppedverified. Goalactive.

---

# Live update — 2026-09-07 22:11 UTC

LATEST USER WORK: recent-paper review and two further native experiments COMPLETE.
Columba2025/b-move2025 notes docs/OPTIMIZATION_RESEARCH.md. Wordcache median
27.1616sbaseline/27.3241scached ->0.9941, left opt-infalse. Extensionreuse8threads
with explicit1k warmups andretainedbinary control:5.4238s/5.5137s ->0.9837;
REMOVED regressioncode from local+EC2, restoredexactpriornativebinary andsources.
Retained source benchmarks/experiments/extension-reuse-source.tar.gz, outcomes
WORD_CACHE_EXPERIMENT.md andEXTENSION_EXPERIMENT.md, fullcomponentiterations.

Native corrected all7classic sizesx3 COMPLETE, all26643witnesses scalarverified.
Largest median1.2336s includingload/output/verification; BWA72.0249s,26019recovered.
NativefinalSHA d30200cf08b80a2d9f235a567679219932cc107dba2ff712f68ffa13171ed1ba
matchesALL21publishednativeiterationhashes (verified). Final provenance local
andremote data/classic-v1/final-native-provenance.json; prior record retained.
Nativepost-classic inventory6052files/105197464023bytes copied data/ec2/.
Native i-0b5bad3102a5345c5 and BWA i-0eed25aed0ad7dd2c both STOPPED, verified22:14.
Do NOT cancelpersistentSpotrequests: earliercancellations terminatedinstances.
Allvolumesretained; failedminimapOOMlogs recoveredlocally already.

README now embeds actual docs/images/classic-scaling.svg andclassic-index-builds.svg,
PNG/CSV companions. Plotsvisuallyinspected; shortenedrecoverylabel/fixedfooterclipping.
Scaling currentlyin-progresscaption, only3completepointspertoolplotted; source
includesall availablecompleteddata andtimeoutlowerbounds. Buildfigure4actualbuilds,
allCPU/RAM/singlebuild/cache/referenceinputcaveats. Docs classicmethods/reprojobs,
ARTIFACTS andVERIFICATION updatedpartly; FINALcompletionaudit stillpending.

Remaining authorizedwork: waitfinish BLAST/minimaplargerbatches, collect/refresh
MD+CSV+figure, updateREADMEremoveinprogressonlywhenactuallydone, finalinventories
andstopactiveworkers (no cancellation/deletion), verifyallstates. Thenassessgoal
complete. BLAST1000/5000 both600stimeout,10000live since22:08:23,26643next,
expectedfinish~22:28:24ifalsotimeout. MinimapmultiplelargerrepsstillLIVE; inspect
structuredscreeningstatuses, no fakecompletedpoints. Bothautomaticstop~00:27/00:33.
ActiveworkersBLAST3.8.2.55/i-043c11d9e3597df11, MM18.130.251.80/i-0b880ceee50e0c3fa.
Source/native13.135.169.26 nowstopping; noinputfetchneededallworkersready.

All19Rusttests+clippy passed afterwitnesscorrection; extensionexperiment19testsalso
passedbutcodewasrevertedusingexacttestedbaseline. Finalclippypassedafterrestore.
Python2verifiertests andallclassicpycompilepass. User-facinglatestupdateexplained
failedoptimizations, stoppednative, waitingfinalcompetitorpoints. Keepupdates~60s.

---

# Live update — 2026-09-07 21:59 UTC

Native resized successfully: i-0b5bad3102a5345c5 now13.135.169.26,
c7i.2xlarge8vCPU/16GiB, shutdown00:47:50Sept8 verified. New index build
206.125s/8625584KiB, complete. Parallel human tuplesk0..3 passed.
Native100pilot FAILED independent distance equality: first path cost can exceed
minimum interval cost. Retained 20260907T215343.555509-pilot-ooff-100-r1-verify.
Fix in fm.rs search_limited reconstructs matched word and normalizes cost with
IntervalDistance. Full19localtests pass; scalar test now asserts every callback's
cost independently, IntervalDistance64base support verified. No validator weakened.

NEW optional --word-cache caches ten-base exact ranges per query during reverse
leaf canonicalization. Default false. Sources docs/OPTIMIZATION_RESEARCH.md.
Research Columba2025/b-move2025; no copiedcode/dependency. Experimentaltests pass.
Staged all source separately data/word-cache-stage. CURRENT native chain PID2719:
results/word-cache-and-native-retry1.log, runs word_cache_experiment.py (tests,
build, exact32tuple comparisonsk0..3, alternatingbaseline/cache1k x3 singlethread)
THEN native_after_gate.py --skip-experiment (new100pilot, seven sizesx3 at8threads).
Old2149 exitedfailedpilot; oldgate1206 complete. Native witness-provenance.json
still OLD binary SHA, update after corrected build with explicit repair provenance.

BWA all7sizesx3 COMPLETE and metadata local. Stop requested21:59, do NOT cancel
Spot request after stop: prior cancellations resulted in TERMINATED instances.
All disks retained. Failed minimap16GiB instance i-0b00bcd23188ec5be TERMINATED,
volume vol-0070954e222f18344 preserved and its failed iterations RECOVERED locally
by attaching to BWA worker read-only/noload, then unmounted/detachrequested.
Earlier unused BWA/BLAST volumes vol-01d47c2593b105cb1 /vol-04240a40cb1554904.
BWA currentroot vol-0fb6f267d2dbd55d9, BLAST vol-08fb4d24762e1d2c4,
minimap32 vol-022afdbf62d1db6cc. No volume deletions.

BLAST full10/30/100 x3 complete, larger runs continuing (likely600stimeouts).
Minimap32 pilot100/100 and fullseries LIVE;10/30/100/1000x3,5000first165.88s.
Minimapbuild73.643s/16122884KiB complete. BWAindex2753.439s/3592708KiB,
BLAST16.004s/90808KiB. New plot_builds.py rendered docs/images/classic-index-builds
SVG/PNG/CSV and visually inspected (actual fourbuilds, all allocation caveats).
Scaling plot labels fixed8CPU; pending native results and all larger comparators.
Docs classic method updated8CPU,isolatedjobs,quotaincreasecommands,indexbuildcaveats.
Recent metadata copied allactiveworkers; regenerateledger as newresults arrive.
Do not stop active native/BLAST/minimap prematurely. Goal remainsactive.

---

# Live update — 2026-09-07 21:46 UTC

Supersedes the older sequential setup notes below. User requests further native
optimization informed by recent papers while independent classic jobs run.
All jobs use eight vCPUs; BWA/BLAST 16 GiB, minimap retry 32 GiB after a confirmed
16 GiB OOM (137, 30.786 s, peak 15,595,688 KiB). Failed raw artifacts remain on
retained stopped instance i-0b00bcd23188ec5be; not yet copied locally.

Native/source i-0b5bad3102a5345c5 stop requested to resize c7i.4xlarge to
c7i.2xlarge. All competitor inputs were transferred. Preserve 200 GiB volume
vol-0138903fbd36c52bf. After restart discover public IP and reset shutdown.
Then run classic/parallel_gate.py (build staged witness binary, tests, fresh
8-thread index build, exact human tuples k0..3), native pilot and measurements.
BWA index completed 2753.43898988 s, artifact 20260907T203303.291193-build-bwa-index.
Old sequential setup40182 and pilot43173 were cancelled; do not reuse scripts.

BWA i-0eed25aed0ad7dd2c /18.170.39.176, Spot sir-zf37hkzq:
100-ASO pilot 100/100, 3.601126888 s including verification. Measurement PID2084
started five sizes100..26643 x3; add10/30 x3 after completion.
BLAST i-043c11d9e3597df11 /3.8.2.55, Spot sir-rfxqkdtq:
index15.98s/90808KiB; pilot search141.035s. Original verification failed on
NCBI gb|rN| IDs; retained, parser fixed/tested, raw output reaudit100/100.
Measurement PID2490 uses seven sizes10..26643 x3, 600s stage timeout.
Minimap retry i-0b880ceee50e0c3fa /18.130.251.80, Spot sir-ky67kvmm:
m7i.2xlarge8/32; pilot PID1799 started index21:38:57. Inspect before full run.
SSH cwd /home/ubuntu/ooff, key data/ec2/controller.key. Workers stop automatically
around00:27/00:33 UTC Sept8; verify and stop/cancel when done, never delete.
Earlier unused16CPU workers stopped, Spot requests cancelled.

Local helper code now supports10/30 batches, query-balanced multiprocess scalar
verification for all four tools, witness mapping, dynamic8CPU settings. Native
witness code staged but not yet built on EC2. Plot labels still stale16CPU;
separate build-time/RAM display and README image remain pending.
Recent Columba2025 and b-move2025 papers opened; no new algorithm implemented yet.
Keep every experimental result (including failures) in MD/CSV. Goal still active.

---

# Live note — 2026-09-07 21:04 UTC

BWA42598 stillLIVE30m41, BWTconstructionFINISHED556iterations1795.64s;
BWTupdate8.06s, packingforwardFASTA phase. Do not restart. Setup40182 and
pilot43173 chainunchanged. LastturnclassifiedPROGRESS+verifiedwait.

BLASTpilot config strengthenedBEFOREfirstmeasurement usingofficialNCBI
supportedscoringtable: reward1/penalty-1,gapopen0/extend2,qcov_hsp_perc85,
evalue1e9; max_target_seqs1000,max_hsps1 retained. This avoids statsignificance
substitutingforthreeeditpolicy. Independentglobalverifierremainsauthoritative.
Allchangesin local+remote scale.py anddocs/CLASSIC_BENCHMARK.md.
No actualBLASTtimingyet; no inventediteration forunexecutedconfiguration.

---

# Latest update — 2026-09-07 20:54 UTC

Previous turn PROGRESS; this turn further PROGRESS: closed native/comparator
output-verification gap. Currentlocal ooff-index.rs QueryRun nowincludes
witness_interval[globalrid,a,b,d] forscreen andemits witness_intervals inrunJSON.
Parallel1/2/16 equalitytestpassedagain;clippy-Dwarningspass. Python2testsnowalso
checknativewitnessesandrejectincorrectdeclarededitdistance.

NEW stagedremote data/classic-stage/ooff-index-witnesses.rs, NOTYETBUILT.
Queuedpilot43173 stillwaitssetup40182. It willsha-checkoldsetupbinary,archive
source-before-pilot, callparallel_gate.py whichNOWcalls prepare_witnesses.main:
copy stagedwitnesssource, touchfile, full19tests+releasebuildlogged, create
native-record-map.json mappingoriginalglobalIDstoeligible rN IDs, validateonly
SCN2A-onlyrecordsareexcluded, writewitness-provenanceJSON(nativebinaryhashplus
mapping/sourcehashes), archive source-with-witnesses.tar.gz. THENfreshSassyvs
native16 fullhumanreport32k0..3 literalchecks, then100ASOphasepilotall4tools.
This additionalPythonstepiscall-time loaded, no needrestartwaitingbash again.
Afternewbuildresults/classic-setup.success oldbinaryhashWILLBESTALE bydesign;
forlaterfullscaleusewitness-provenancebinaryhash+parallel-human-gate.success.
Do notrerunsetup/pilotblindly; mapopenx/oldshagates intentionallypreventoverwrite.

scale.py now post-verifiesALLFOURtools using same scalar verify.py. Native
runJSONactualwitnesses checkedthroughnative-record-map ontoeligibleFASTA bases;
fullquerydistance mustequalnativeclaim. Primaryplottime sumssearch/conversion/
independentverification forALLtools. No easiercount-onlynativecomparison.
command.refresh now emits structuredtool,n,rep,phase,threads columns and
composite screening.csv+MD table as measurements finish. Scriptsstagedremote.

BWA42598 verified20:53:~20m33elapsed99.1%CPU,3.4bncharactersprocessed (BWT
containsbothorientations,expected~4.88bn total). CPU/memorynormal. No search
pilotrunningyet. Newsetupstages/3indexbuilds remainsequential. Stop23:47UTC.
No fullscaleloopqueued; inspectpilotoutputthenstart3repincreasingNexperiment.

Localdata/classic-v1 nowhasall10queryFASTAfiles+manifest; allSHA/counts/RC
orientationsverified. data/ec2/artifact-inventory-before-classic.json copied.
ClassicprepMD+CSV local4completedsteps; rawmapperstdoutnotcopied. Finaloriginal
ledger107complete includingcold. Currentreadme10.67fulloutputproofunchanged.
No READMEimageyet. GoalACTIVE, allremainingdeliveryrequirementsstillapply.

---

# Latest state — 2026-09-07 20:41 UTC (supersedes older notes)

PROGRESS: original final validation chain COMPLETE. Literal135,340,605 tuples
unique/equal, full CSV median native30.47657 vsSassy325.31181 =10.67416x;
firstbatch10.4411x, process10.5734x. Artifact20260907T200643-fm-word-union-full-tuples1000.
All final5screens pass14–185x. Cold1k firstprocess native21.56 vsSassy18.71s:
NOT10x cold. Native21.2726search+.2389load; Sassy.8269search+17.6953load.
README updated exact final10.67 result and explicit cold limitation. 107 rows
metadata synced locally, final inventory generated on worker (notyetcopied).

NEW USER DELIVERABLE stillINCOMPLETE: README measured benchmark image with3–4
classic tools, increasing ASO counts,16cores. User explicitlyexcludedBowtie;
we excludedbothBowtie1/2, selectedBWA-aln,BLASTN-short,minimap2. Noagents.
Askedasyncscreen/allsitesandreference scope; unanswered. Statedassumptioncurrent
Ensembl110 RNA records,screenonewitness. MustlabelderivedRNA,notwholeintergenicDNA.
Details docs/CLASSIC_BENCHMARK.md; use actualdataonly, recoverybesideelapsedtime.

Implemented--threads in ooff-index benchmark ONLY; scopedthreads sharemmap,
perquerydisjointresults, tuplewriterMutex; preserves1thread directworkerpath.
ProductionannotatedCLI remains1thread. All19Rusttests passlocal+EC2 (new1/2/16
worker sites/screen at k0..3). Localclippy-Dwarningspass. NewPython scalar
mapperintervalverifier2testspass (strand,XA,unknowns,terminalgap,CIGAR).
Oldsinglethreadbinarysavedremote data/classic-stage/saved-binaries/ooff-index-single-thread.
Newsource staged/builddone, do notoverwritecurrentbinary witholdercode.
Humanparallel32fulltuplegate queuedaftercurrentindexbuilds beforepilot.

Live setupPID40182 (parent40181), logresults/classic-setup.log.
Waitedfinalinventory38679 nowcompleted, grewrootvolume100->200GiB safely.
Allrawartifactsretained; ~105GBfree. Tools aptinstalledbwa, ncbi-blast+,minimap2.
Native19tests/releasebuilddone. Prepared data/classic-v1/eligible.fa:
2,438,579,557bases,288,398records,excluded21SCN2A-onlyrecords; hash
 d5a21da4ce52afb116b0c5d1a5ec9a5c14a872ee43b6b2b16527ff0ed48dd16b.
Nativeoriginalindex+gene filter searches SAMEeligibleRNAs. Comparatorrefsingle
line/record withrNids andoffsetmetadata for independentverification.
Nestedactualhistoricalallelepool queries100/1k/5k/10k/26643, originalASOandRCFASTA.

BWA INDEX LIVE42598, command bwa index -p data/classic-v1/bwa eligible.fa.
Lastverified20:39:~6m47elapsed97%CPU,1.2bn BWTcharactersprocessed.
Artifact20260907T203303.291193-build-bwa-index, timeout5400. Sequential next
BLASTdatabase, thenminimap2k7w5index16threads(timeout1800each). 32GiBmemory
maylimitdenseminimapindex; inspectfailure,retain,tunerespectfully ifneeded.
Setupends toolversions+nativehashmarker results/classic-setup.success.

Queued PILOT NOW43173 waits40182, logresults/classic-pilot-retry1.log.
Oldwaitingpilot42825 stoppedbeforeanymeasurement so updatedshellscriptisloaded.
Newpilot first runs parallel_gate.py: freshSassy1thread vsnative16threads full
humanreport32 k0..3 literal equality/uniqueness, resultsgate required. 100ASOs1repall4
engines,phasepilot,timeout600perstage. NOfullscaleloopqueuedyet; inspectpilot
recovery/speed beforelaunchingfull3rep100/1k/5k/10k/26643. Sourcearchiveatpilot.
Scripts command.py retainEVERYinvocationJSON/log/time+MD+CSV, hash executable
andscripts; scale.py linksstages in screening.json. verify.py scalarchecksactual
sourceintervals d<=3, plusorientation, noN;doesnotclaimallsiteenumeration.
BWAaln16threads+samse1000XAcap,BLASTNshort16threads1krecords/1HSP, minimap2
k7w5m10s10n2 plus-only100secondary. Heuristicsmustshow recoverylimitations.
plot.py requires3repsat>=3sizesforall4tools, thenrendersSVG/PNG+CSV; notrunyet.
Local matplotlib installedin /tmp/ooff-plot-venv; nofakeplaceholderimagecreated.

Worker i-0b5bad3102a5345c5 /16.61.245.106 c7i.4xlarge16logical/8physical32GiB.
Retainedvolumevol-0138903fbd36c52bf now200GiB. Auto-stop23:47UTCSept7verified.
Stopsoonerwhenallworkcomplete; no deletions. SSHkeydata/ec2/controller.key.

Oopsfirstmetadatarsyncexcludedwrongbucketdirname andcopied~3.7GBoldrawbuckets
local; stoppedtransfer, retainedpartialcopies, added.gitignorerawbucketpaths.
Correctsyncexcludes *.csv, *.jsonl, site-comparison-buckets/. No deletions.
Futureclassicmetadataexclude stdout(largeSAM) butinclude screening.json,witnesses,
result.json,stderr,resources,started; keep rawvolume. Copyquerymanifests/inventory
and update docs/reference.md cutoff(currentlyolder20:47) beforefinalhandoff.
GoalACTIVE becauseaddedfigure/scaling deliverablesremainincomplete.

---

# Latest state — 2026-09-07 20:28 UTC (supersedes older notes)

NEW USER SCOPE: README benchmark image, ooff + 3–4 classic tools including BWA,
16 cores/threads, increasing ASO batch sizes against human genome. Async workload
question pending (screen vs all sites; RNA records vs whole primary genome).
User explicitly requested omitting Bowtie. Omit Bowtie; conservatively also
omit Bowtie2. Choose three other gapped comparators including BWA; measure
recovery and explain differing gap/
reporting semantics. Current native single-threaded; implementing real scoped
query parallelism for benchmark entrypoint, preserve old running binary.

Dedicated worker i-0b5bad3102a5345c5 / 16.61.245.106 still 16 logical CPUs,
8 physical, 32 GiB. Automatic stop extended and VERIFIED to 23:47 UTC Sept7 for
new authorized scope. Volume expansion requested 100->200GiB retaining artifacts;
inspect AWS result and grow partition/filesystem after current timed jobs.

Round14 retry COMPLETE: new range-union 1k full sites native27.6119/24.7833/
24.7910s vs fast-interval Sassy349.0324/319.2264/320.7060s =>12.9364x median.
45,113,535 counts/signatures equal. All18 EC2 tests passed, report32 k0..3 literal
site equality, annotated CLI 3runs all fields equal and all tuples audited.

Round15 retry PID38066 currently active, runner39255, label
fm-word-union-full-tuples1000. Sassy FINISHED successfully 983.03s at~20:23;
native and 135,340,605 literal tuple comparison next, then five screen sets.
Do not overlap compilation/indexing/timers with this chain.
Queued final cold PID38621 waits38066, requires all6 final results valid, then
ONE cold original1k screen repetition. Queued inventory PID38679 waits38621,
writes artifact inventory and ledger. Old inventory38147 stopped intentionally.
No auto-stop in these scripts. Recheck success before new classic jobs.

Local metadata100 completed iterations; newest remote results need syncing.
Raw large outputs retained EC2 only. README headline still older11.44x pending
latest full tuple check. GoalACTIVE: do not claim cold/end-to-end universal10x.
No subagents authorized; Sassy is external comparator ONLY. No deletions.

---

# Latest state — 2026-09-07 19:50 UTC (supersedes older notes)

Turn PROGRESS: strongbaselinefullreport finished11.44x; diagnosed/fixedstagedCargo
mtimebug; recordedfailedbuild+cancelledinvaliddownstream; newrangeunionEC2tests
andk0..3exactsitechecksPASS; strongerfull1knewalgorithm nowLIVE. GoalACTIVE.

ROUND13 COMPLETE 20260907T192353-fm-packed-fast-interval-sites1000:
Sassy320.8493/320.3661/320.5519s (engine311.0061/310.5192/310.7005),
native36.7409/28.0194/28.0268s =>11.43734x median, all45,113,535counts/signatures.
Native top10queries18.5%oftime despite54.3%sites; hashentrypackingonlymodestgain.

ROUND14 originalFAILED becausecp-a preservedstagedFMsourceolderthanlastCargo
build; Cargo reusedoldlibrary whilecompilingnewbin ->search_unique notfound,
eventhoughsourcehashcorrectcontainedmethod. Downstreamround15 incorrectly
startedoldbinary underwordunionlabel. Intentionallystopped itsrunner/process
andqueuedinventory; verifiedalldead19:45:17; keptpartialfiles/logs.
Recorded nativebuildfailure20260907T194149-native-range-build-failed and
aborted20260907T194149-fm-word-union-full-tuples1000 (NO native measurement).
No deletions. Oldlogfm-round14.logkept, retrylogsnewnames.

FIX: round14 touchesONLYsrc/tests aftercp-a toforceCargo sourceinvalidation;
actualcorrectbuild/testsPASS. run_iteration.py nowexit1afterrecordinganyfailed/
mismatchedcomparison. round14 writesincomplete markeratstart, writesnative
binarysha256successmarkerONLYafterallchecks; round15 verifiesmarkerbeforetiming.
Scripts15+finalinventorynowtake predecessorPIDargument. Preventstale-binaryruns.

LIVE ROUND14 RETRY bashPID36335, logresults/fm-round14-retry1.log.
Newrangeunion all18testsPASSonEC2; exactreport32k0/1/2/3allPASS. k3firstnative
.881344 vsSassy12.731913 =>14.446x (1rep). No warmmedianclaimforthatshortcheck.
FULL1k runner38223/Sassy38225 verified19:49:31 at99.9%CPU elapsed1:06;
label fm-word-union-sites1000,3repsnoCSV,timeout1200. Start~19:48:24.
Afterfullpair,scriptCLIreport32threeprocesses+allJSONfieldandSassytuplesaudits.
Nativequery_seconds nowONLYreports(clockprofilingremovedfromscreen),staged14
beforecorrectedbuild,clippypassed. Nootherfunctionalchangesbeyondpreviousrangealgorithm.

ROUND15 RETRY queuedPID38066 witharg36335,logresults/fm-round15-retry1.log.
Willrequire results/fm-round14.success SHAchecks, thennewrangefull1kCSV3reps
+exact135.3mrowcomparison(timeout1200),freshscreen10k/mixed/random/lowcomplex/
allallele26643matrix. Currentcomparatorstillfast-interval variant rebuilt.
Finalinventory queuedPID38147 witharg38066,logfinal-inventory-retry1.log;
willwrite results/artifact-inventory.json andrefreshremoteledger afterqueue.
OldinventoryPID35580 andoldround15PID34874 intentionallyterminated, donotwaitthem.

AUTO-STOP20:47:00UTC stillverified, sameworker i-0b5bad3102a5345c5/16.61.245.106,
volumevol-0138903fbd36c52bf retained. Expectedremainingtests~40min, plentywithin
cutoffunlessnewfailures. Stopearlierwhencomplete; neverdeletefiles/resources.

Localmetadata synced93completedrows at19:47, remote96latest. refreshledgerlocal
nowexplicitABORTEDstatusforcancelledrun (notyetstagedremote latesttinychange).
READMEstrongbaseline11.44 updated, BENCHMARKSfailureexplanationadded,
docs/ALGORITHM.md exactrangeunionargumentadded, docs/ARTIFACTS.md inventorymap.
Inventoryhelpernowincludesexternalcomparators+roundstages/savedbinaries andis
stagedremote; controllercopyquerysets/manifest/logs/CLIledger stillpendingquiettime.
No rawlargeCSV/JSONcopiedlocal; retainedvolume pathsdocumented.

Next: inspectnewrangefulltimings thenfull135.3mexactproof/screenmatrix,
finalauditlatest18testscope+annotatedfields, docs/resultsinventorysync, stopworker
verifyAWSstate. Goalnotcompleteuntilnewstrongfull-outputcaseverified>=10.

---

# Latest state — 2026-09-07 19:29 UTC (supersedes older notes)

Turn PROGRESS: round12 full135.3m exacttuples PASSED; strongercomparator
report32 PASSED20.51x; verifiedlivefullreport; extendedboundedworkerdeadline
withapprovedtool; artifactretentionmap/inventoryhelperadded. GoalACTIVE.

AUTO-STOP NOW20:47:00UTC, verified /run/systemd/shutdown/scheduled USEC1788814020000000.
Original19:47cutoff extendedonehour at19:25 on SAME dedicatedworker (i-0b5bad3102a5345c5,
16.61.245.106, eu-west-2, retainedvolumevol-0138903fbd36c52bf). Explicittoolapproval
succeeded, noongoingpermissionblock. Stop soonerwhencomplete, retainallfiles.

ROUND12 FINISHED: 20260907T185255-fm-final-exact-tuples-1000-repeated.
3reps45,113,535each, total135,340,605 literaluniquetuplesperengine equal.
Native64.2377/34.3797/34.3041s, Sassy472.7190/444.3305/443.6856s.
Median12.9242x INCLUDINGCSV, overallprocess10.2297x, oldscalartracebackverifier.
Newstrongerverifier stillmust bechecked; doNOTuseoldresultfinalacceptance.
Localmetadata/source sync88completedrows inclfullround12proof. Remote89now.

ROUND13 LIVE: runner34762/Sassy34764 verified19:28:30 at99.9%CPU elapsed4:37.
Label fm-packed-fast-interval-sites1000, three repsnoCSV,timeout1200perengine.
Bash30663. Precedingreport32 20260907T192251-fm-packed-fast-interval-sites32:
newdistance-onlySassy12.75564s/nativecompact.621927s =>20.50986x, exacttuplespass.
Fastcomparator /home/ubuntu/comparators/sassy-0.2.6-fast-interval/target/release/sassy-comparator,
SHA d7b90cf2090c97b240de062c7bf84069519990e76a43e7cebbcd0342cc9e3566.
Fullreport1000 resultsinclude perrunSassyengine_seconds/nativequery_seconds.
Inspectthese, notspeculation, toidentifycost. It mayapproach1200timeout;
ifitfailskeepattemptandrerunwithsufficientbound, don'trestartliveprocess.

ROUND14 queuedPID31111 waits30663, newrangeunionstaged independently.
Newcurrentlocalcode tests18passed/clippypassed, no newcodechanges thisturn.
Willtestbuild, k0..3report32exact, full1k3repnoCSV(timeout1200), CLIreport32
3runs+completeJSONfieldaudits. Runtime/genes/coordsunchangedbywordunion.

NEW ROUND15 queuedPID34874 waits31111, scriptfm_round15_ec2.sh/logfm-round15.log.
No compilation. Newrangeunion full1kCSV3reps+exact135.3mrowcomparison(timeout1200),
thenfreshscreenmatrixscn2a10k/mixed1k/random1k/lowcomplex1k/allalleles26643,
all3repssamefastSassycomparator. Disk26GBfreebeforethisnextCSVjob (~16GBextra).
Checkdiskandtimebeforebigoutput; nofiledeletions. Canpreserverawremotely.

Localnewbenchmarks/inventory_artifacts.py and docs/ARTIFACTS.md added. Inventory
stat-only(no hugecontentreads), includesiterations/results/publicreference/raw,
querysets,indexes,staging/savedbinaries, externalcomparators. Notyettransferred
or runonEC2; doaftertimingquiet. Localinventory /tmp/ooff-local-artifact-inventory.json
current878files228MB (excludesheavyrawCSV/JSONoutputs keptEC2).
Localrefresh_ledger adds sassy_engine_seconds,perenginefirstsearch times toCSV;
notyetremotelysynced. docs/reference.mdupdated20:47cutoff. Need syncfinalmetadata,
CLIledger,logs,querymanifests/data (data/query-sets-v1 actuallyABSENTlocal despite
oldernotes); retainrawbigfilesonvolumeandprovidepaths/inventory.

Next: inspectround13->14->15, adjustbasedonrealfastbaseline, finalscopeaudit
includingnewcorefulltuples+allfields+18EC2tests, finaldocs+stopworkerverify.
Noagents/nodeletions. Existingusergoalnotcompleteuntiladequateevidence.

---

# Latest state — 2026-09-07 19:15 UTC (supersedes older notes)

Turn classified PROGRESS: implemented and tested range-level canonical-word
union to eliminate per-occurrence site hash tables in uncapped reporting.
Goal remainsACTIVE; no performanceclaim fornewalgorithm untilEC2results.

Currentlive ROUND12 runner30050/Sassy30053 verified19:14:43 at99.9%CPU,
elapsed21:47. bash30047. Three reps45.1mCSV each withOLD scalarverifier,
thenoldnative3reps+literalbucketcomparison. Logresults/fm-round12.log.
ExpectedSassyfinish~19:16 thennative/comparison. Auto-stop19:47:43 stillset.

ROUND13 queuedPID30663 waits30047. STAGEFROZEN atdata/round13-stage:
compactSiteSet, fastIntervalDistancecomparator, ddgmetadata, 18tests.
Also addedquery_seconds perquery arrays to nativebenchmark for profiling.
Compiles/testsonEC2, thenfastcomparator report32 exact3reps+report1000counts3reps.
Do notoverwrite stage13 withnewrangealgorithm; thisisolatescompactstorageresult.

NEW ROUND14 queuedPID31111 waits30663; scriptfm_round14_ec2.sh,logfm-round14.log.
STAGE atdata/round14-stage has CURRENTlocal src/tests (newrangealgorithm).
Will retainoldnativebinary, installstage,run18tests,buildnative+rebuildfastSassy
externalcomparator. Thenexactreport32k0..3each1rep, report1000counts3reps,
productionCLIreport32threeprocesses+fullJSONfieldcomparisonagainstborrowedJSON
baselineandfullintervalauditsagainstnewfastSassy. No newfull45mtuples yet;
need one fulltuplevalidationofnewalgorithm afterperformanceinspection.
Bothqueuedstages useproductionforward/reverseindexes unchanged.

Newalgorithmdetails:
- Refactored FMvisit leafcallback to emit(lo,hi,span,cost,word), whereword stores
 encoded matchedcharacters right-to-left in shared128cellstackbuffer.
- Existingsearch_limited wrapsrangeleafcallback withlocate loop, preserves
 witness/earlystop semantics. Wordwritescanbeoptimizedawaywhenunused.
- search_unique(pattern,k,optionalreverse,emit) collectsminimumcost forward
 ranges andcanonicalizesreverseleafwords viaforwardFMbackwardextensions.
 Mergekeys(lo,hi,span). Distinctsame-lengthwords have disjointSA ranges,
 so expand each finalrangeonce: exactminimumcost, duplicatefreeoccurrences.
- Fulluncapped productionreport andnormalbenchmarksites use search_unique,
 emitdirectly withoutSiteSet. Production stillindependentlyscalarverifies
 everyactualsourceinterval beforealignmentannotations. Screen/cappedreport
 retainoldstreamingpathwithcompactSiteSet. Automaton/distance-strata retained.
- Tests/fm.rs nowchecksunique-rangeoutput(singleandpaired) againstindependent
 scalaroracle, assertszero duplicatecallbacks, allk0..3. ExistingwrappedFASTA
 cached/pairedCLItestscoverperrecordreverse/remapping/annotations.
- ALL18tests PASS localafteralgorithmchange; clippy-Dwarnings PASS after
 equivalentcollapsible-ifformatfix(staged). NoEC2measurementyet.

Evidenceforoptimizationtarget: top10queriescarry54.2636% of45,113,535sites.
Largestquery357AATAT... has3,941,769; query281GTGT...3,643,229. Manyrepeatword
occurrences formerlyenteredhashmapseparately despiteidenticalalignmentword.
Newrangeunioncanavoidmillionshashoperations whilekeepingallrepeatmatches.
No datafiltering/strandexclusions added.

Next: monitoroldrun+queuedfairercomparators, inspect perquery/engine_seconds,
optimizebasedonfreshresults, full45mexactvalidationnewcore, scopeaudit/docs,
retainartifacts/syncledgers/stopdedicatedworkerwhencomplete. Auto-stop~32minaway;
checkactualprogress before anydeadlinechange. Noagents/no deletions.

---

# Latest state — 2026-09-07 19:05 UTC (supersedes older notes)

Turn classified PROGRESS: found a material comparator fairness issue, implemented
and verified stronger interval distance, compacted native site storage, added
missing optional ddgmetadata support, and wrote docs/VERIFICATION.md audit.
Goal remains ACTIVE. Do NOT accept earlier16x report claim as final yet.

ROUND12 stillLIVE lastverified19:02:57: runner30050/Sassy30053,100%CPU,elapsed10min.
Bash30047. Three repsCSV45.1m each, then exactcomparison. Logfm-round12.log.
Original slow scalar traceback comparator stillused intentionally for archived
runningiteration. Do notinterrupt/restart it oroverlapbenchmarks.

NEW ROUND13 queued PID30663, waits kill-0 bash30047. Scriptfm_round13_ec2.sh.
Source/tests/helper staged data/round13-stage, doesNOTtouchlivefilesyet.
Will retainoldnativebinary data/round13-stage/ooff-index-before-packed,
copy stagedsrc/tests/helper, full18testsEC2, buildnativeCLI+index,
then separateSassycomparison /home/ubuntu/comparators/sassy-0.2.6-fast-interval.
Runs freshreport32 exactCSV3reps, thenfreshreport1000 noCSV3reps(timeout1200).
Defaultsupported Sassy2Iupacbatched; no upstreampatches,AVX512knownbroken.
Auto-stop19:47:43UTC; evaluate remainingtime, do notleaveunfinishedjobsunrecorded.

Criticalnewfairnessfinding: engine_driver.rs previouslycalledooff::align for
EVERY candidateinterval, producingunusedtraceback. Now usesIntervalDistance
precomputedquerymasks(globalMyers lowPhbit1, unlikeinfixlow0), ~20bitupdates.
This canmateriallystrengthenSassyreporting; earlier16xmaynot survive.
Sharedbenchmarkhelper also recordsengine_seconds aroundfind() calls soactual
Sassysearchcostisdistinguishablefrom verification/output; speedupstillfullsearch.
No claimfastestvalidcomparatoruntilthismeasured. Native scalarCLIalso usesnew
verifier torejectbadintervalsbeforefullannotation, keepsscalarfinalverification.

NativeSiteSet in fm.rs now FxHashMap<u64,u64> vsfour-wordkeys/values:
keyabsolute shardstart<<8|span; valuerecordlocalno<<8|minimumcost.
Uses u32shardcoordinates, assertsvalidbounds. sitesuniqueness/mincost preserved.
BothindexbenchmarkandproductionindexedCLI useit, decodingglobalrecord/coords
at emission. Existingk0..3 andwrapped/pairedcacheCLIoraclesallpass.
Need freshhumanexactsetandtiming afterchange; currentround12 testsOLDbinary.

Queryoptionalddg:Option<f64> added becausehandoffexplicitlyrequireditifprovided.
SiteOutput preservesit onlywhenSome, no filtering; missingfieldoldoutputunchanged.
Newenergytest99999.25 stillsame sites,fieldretained. NativeREADMEdocumented.
18tests PASS local; clippyalltargets-Dwarnings PASS afterallchanges. EC2testsqueued.

LatestlocalBENCHMARKS includesprominentfairnessupdate; docs/VERIFICATION.md matrix
listsactualremainingwork, donotmarkgoalcompletebasedonlyonoldwarmmeasurements.
Next: inspectround12 thenround13 results, stronglyoptimizeifnewSassybaseline
makesratio<10, continueaccurateledger, finalfulltuplevalidationnewcore,
finalartifactinventory/sync/docs/stopworkerretainingvolume. Noagents/nodeletions.

---

# Latest state — 2026-09-07 18:55 UTC (supersedes older notes)

This turn made concrete progress: all negative controls k0/1/2 matched,
optional upstream AVX512 build was attempted and failed (SIMD u64x8/u64x4
errors), annotated CLI serialization optimized and all fields verified.
Goal remains ACTIVE; do not mark complete yet.

ROUND12 LIVE: bash PID30047, runner30050, Sassy30053 verified18:54:58 at99.9%CPU,
elapsed2:03. Label fm-final-exact-tuples-1000-repeated; start18:52:55;
log results/fm-round12.log. Three reps full CSV tuples45,113,535 perrep,
then native three reps and complete literal 128-bucket comparison. Uses
supported original Sassy2Iupac comparator; timeout1650 eachengine. Expected
~25minSassy +2minnative+severalmincomparison. Auto-stop19:47:43; disk39GBfree
before fulloutputs. DO NOT compile/run another timer or large data transfers
while this is active. Native ooff-index binary unchanged from round8.

Round10 optionalSassyavx512 failed to compile upstream, so its timing commands
never ran. Failedbuild explicit iteration20260907T185138-sassy-avx512-build-failed,
fullcompilerlog+comparatorCargo.lock/source archive retained. No sourcepatching.
DefaultfeaturesonlyCLI/diagnostics; scalar not enabled; target-cpu=native used.
Round11 completed; main.rs now emits serde Serialize borrowed SiteOutput,
avoids Value/recordmetadata copies. All16tests passed local AND EC2, localclippy
passed. Native CLI binary rebuilt; ooff-index release code unchanged. All78,733
annotatedsite JSONobjects exactequal oldoutput (canonical fullfields Counter),
allintervals exactequal Sassy, perrun auditJSON retained. CLIreport32 wall
3.2215(first)/1.7181/1.7186; search+output2.4097/1.1740/1.1645s,64.9MB each.
Old warmedCLIwall1.9686 =>modestrealimprovement; no10xCLIclaim.

run_cli_iteration.py now supports --queries-jsonl preserving allele metadata.
Fullhistoricalallele26643 CLI screen:2.3701swall,1.9257ssearch+output,30.1MB,
all26643sites/queries emitted; allallelemetadata andsummariesaudited.
Data provenance unchanged; olderpoolnotlater6338eligibleexport.

Local metadata sync complete87iterations, plottingCSV regenerated. NewCSVcolumns
query_file,repetitions,writes_site_csv,cache_policy,sassy_binary helpgroupplots.
refresh_ledger.py newcomparator_build rowtype implemented andrunlocally; remote
has supportbut NOT latest5CSVcolumns yet. Sync helper aftercurrenttiming ifdesired.
LocalBENCHMARKS/README/HANDOFFupdatedAVXfailure+CLIresults, nooverclaimcold10.
RawCSV/buckets/outputJSON remain onretainedEC2volume, metadata/sourceslocally.
Need finalartifactinventory/copyplan, sync latestresults+CLIledger/logs,
then evaluate3repfullCSVperformanceandfurtheroptimizeifunder10.
Need finalscopeaudit, update docs, stopdedicatedworkerwhenworkcomplete,
retainvolume/data(no deletions). Goalnotcompleteuntilevidenceadequate.

---

# Latest state — 2026-09-07 18:47 UTC (supersedes all older notes)

Goal remains ACTIVE. User asked why reverse index is needed; explained that
reversing BOTH query and reference preserves the same biological pairing, and
only accelerates traversal. No cancellation of benchmark objective.

Round8 finished; fresh report1000 three runs: native 118.727/29.099/29.103 s,
Sassy 438.245/470.442/469.823 s. Median 16.143×, FIRST batch only 3.738×.
Round9 completed allele pool screen1k/10k/all (68.77/89.88/119.66× medians).
Exact full tuple comparison PASSED: 45,113,535 literal unique rows per engine,
128 retained comparison buckets. Native81.639 s vsSassy451.664 s =>5.532×,
first run, so do not attribute difference solely to serialization.
Random negative-control k0 completed native.0602/Sassy174.12 =>2890.8×;
k1/k2 still queued/running under round9 PID25984, check log results/fm-round9.log.

NEW round10 queued, waits for round9 PID25984, script benchmarks/fm_round10_ec2.sh.
Builds separate official Sassy avx512 feature comparator in
/home/ubuntu/comparators/sassy-0.2.6-avx512 (same source, no upstream edits).
Previous comparator default-features=false disables CLI/diagnostics, but AVX512
is explicit opt-in even with target-cpu=native; this new fairness check is needed.
Then exact report32 three reps, allele-all screen three reps, full report1000
WITH CSV tuples three reps + complete literal comparison. Timeout1800 perengine.
No concurrent compilation/timing. Log results/fm-round10.log was empty18:45:56
because waiting. SSH launch session83175 remains open due background shell fd;
a chained rsync will run after round10 exits (harmless duplicate sync).
Standalone metadata/source rsync session63777 completed successfully. Local79
iterations + updated plotting summary.csv; excludes *.sites.csv,
site-comparison-buckets/, output.jsonl, all retained on EC2 volume. Disk40GBfree
before new3rep tuple job. Auto-stop19:47:43UTC, approximately1hremaining.
Need retrieve raw artifacts or clearly record retained volume paths before stop.

Local cargo clippy --locked --all-targets -- -D warnings PASSED18:44. Full16tests
already passed local and EC2 round8; only later native change moved cfg(test)
module to end of lib.rs to satisfy clippy (no release behavior changes).
README/HANDOFF/BENCHMARKS/docs/reference updated with measured scope/cold caveats,
annotation-cache command and historical allele pool provenance. No final claims
until AVX512 checked. BENCHMARKS API audit paragraph still needs updating from
v1 pending to observed v1 Iupac slower and v1 DNA batch unsupported.

Recovered local historical cache /home/barneyh/dphil/paper3/data/genes/SCN2A.npz:
1340targets (1256SNP84indel), source cache SHA cf6a80cf42db7b47b5c6fd21770d0c37aec114ab7e9022a9f7da052350684be4.
AllREF validated exact Ensembl110 gene. make_cached_allele_queries.py generates
26643unique20mers/26940associations, no newDDGfilter, no sample/haplotypeexports.
Data local+remote data/cached-allele-queries-v1; same availableactual32 shortlist
also fulltuplechecked. This older pool is NOT later6338 eligible export.
Two automatic review rejections of public/syntheticdata transfer resolved by
read-only provenance +AWSownership evidence and approved direct retries. NO
ongoing block. Do not repeat permission questions.

Worker16.61.245.106, i-0b5bad3102a5345c5 eu-west-2; volumevol-0138903fbd36c52bf.
Stop dedicated worker when all work complete; never delete files/resources.
No agents authorized. Sassy external comparator only, never native dependency.
Need finish AVX512 result inspection, further optimization if warranted,
updated final ledger/docs/artifact retention and honest goal assessment.

---

# Latest state — 2026-09-07 17:44 UTC (supersedes notes below)

Goal ACTIVE; do not mark complete yet. Worker started~15:47, auto-stop19:47:43.
Current fresh report1000 runner PID20537, SassyPID20539, started17:35:43;
log results/fm-fresh-sites1000.log, one repetition timeout900. Last checked
17:44:17: Sassy still99.6%CPU, elapsed8:33. Its predecessor needed~529seconds.
Queued process PID20825: `benchmarks/fm_round7_ec2.sh`, waits for20537 then
compiles, builds annotation cache, times/audits cached CLI screen1k/report32
three times, CLI screen10k, broader generated query matrix, actual allele
shortlist32 full tuple comparison, v1 screening comparisons, controlled cold
screen1k. Source and scripts are staged. Avoid overlapping EC2 compilation/timing.

New evidence since earlier notes:
- Full report1000 =45,113,535 sites; max per query3,941,769. BTree dedup
 120.494s vsSassy522.237s =4.334×, allcounts/signatures matched.
- FxHashMap site dedup: median86.923s over3reps,6.008× against retainedbaseline.
- Whole FM-range dedup at zero-budget exact tails: median33.650s over3reps,
 15.520× against retainedbaseline, all45.1m sites matched. This is the current
 core. It stores final (lo,hi,span) in existing state memo BEFORE enumerating
 SA occurrences, eliminating repeated occurrence-level callbacks.
- Fresh pair above is verifying that improvement without reusedbaseline.
- Full tuple equality (not just additive signatures) passes report32 k0..3.
 Annotated CLI report32 fully matchesSassy tuple set, unique, validblocklengths.
- Earlier paired CLI report32:3.2596s total,1.8476search+output (beforecache).
- Sassy API audit: batched v2 encoded API IGNORES without_trace() flag in0.2.6.
 DNAprofile still crashesTrace failed; failedattempts sassy-dna-notrace-* logged.
 Restored workingIupac batched API. Separate officialv1DNAwithouttrace comparator
 now at /home/ubuntu/comparators/sassy-0.2.6-v1/target/release/sassy-comparator.
 v1 fullreport32 exacttuples matched but45.332s vsbatched~13.3s; batchedfaster.
 Default comparator restoredIupac, named sassy2-0.2.6-iupac-batched.
 Never addSassyproductiondependency; no upstreamSassy modifications.

Current code (staged on EC2; cache code not compiled there untilround7):
- AnnotationCache::create validates allrecords once againstsourceFASTA metadata,
 records input SHA,size,mtime,versionedJSON offsets. open_cached_immutable
 rejects changedfiles, loadsrecordannotations lazily intoOnceLock<Box<Record>>
 only for emittedrecords; validates each decodedrecord again. Query eligibility
 uses validatedFASTA geneheaders. No record/sequence coverage is skipped.
- CLI --annotation-cache PATH optional; ooff-index cache-annotations builds it.
 run_index_build.py supports --annotations/--index to logcachepreparation separately.
 run_cli_iteration.py supports --annotation-cache and--reverse-index, retains
 timed fullJSONL,handles/records timeout/partialline,refreshesmainMD+CSV.
- All prior14tests passed aftercore rangechange. CachedCLI equality k0..3 and
 changedannotation rejectionpassed; all-targets latestsession93022 running/possiblydone.
 clippy49616 running/possiblydone; recheck. Most recentcacheformat/offsetchecks
 added and fixture test82923 running/possiblydone. No timingneededlocally.
- cache_state.py evicts ONLY benchmarkinputfiles withfadvise, checksremaining
 residencyvia mincore. Tiny localfixture verified0residentpages. --cold usesit
 beforeeachengine; do not call ordinaryfresh-process runs trulycold.
- runner --reuse-sassy explicitly retains identical-workloadrecord fordevelopment;
 finalacceptancefreshpairs. Logsarchiveactualexternalcomparatorvariant source/lock.
- CSV nowhasperengineRSS,wall,indexbytes,outputbytes,counts,exacttupleequality,
 reusedbaselinepath. BothMD+CSV includeproductionCLI/indexcachebuild rows.
- BENCHMARKS.md interpretation documentsAPIaudit, historicalfailures,reusedbaseline.
 Local artifacts lastsync~45rows; remote~51+; syncagainandregenerate localledger.

Still pending: inspect fresh1000 andround7 results; fix anybelow10validcomparator
 cases, quantifycoldvswarm, verifyactualcachedCLI speed/output. Actualfull6338
 allelepoolpath asyncquestion stillpending; availableactual32shortlist is in
 benchmarks/sassy_benchmark/SCN2A_patterns.fa withqueries.csv metadata. Keep
 scopeexplicit anddo notpretendgeneratedgene10k areallele-specific. Run forfewh,
 retainallresults,finalfreshverification/docs/stopdedicatedworkerwhencomplete.

---

# Active optimization state — 2026-09-07 17:07 UTC

Goal remains active: verified 10× versus external Sassy, realistic human inputs,
correct annotated CLI, retained iterations. User expects a few hours of work.
Sassy is only a comparator on EC2. Never add it to native dependencies.
No deletions. No agents authorized. Stop dedicated EC2 when finished, retain disk.

Worker: ubuntu@16.61.245.106, instance i-0b5bad3102a5345c5, eu-west-2.
SSH key data/ec2/controller.key, known_hosts data/ec2/known_hosts.
Root /home/ubuntu/ooff. Auto-stop verified scheduled 19:47:43 UTC.
Other workers must remain untouched. ~56 GiB free before reverse index build.

## Latest evidence

- FM exact-tail/FxHashMap screening: 63.9× SCN2A 10k, 79.4× random 1k,
  15.0× low complexity 1k. Same one-thread comparator policy/reference.
- Ordinary two-shard report32: 2.1723s versus Sassy13.2620s =6.105×.
- Experimental Myers automaton preserved all 78,733 sites but regressed:
  4.64×, then prune-before-rank5.01×, band pruning5.16×. Keep optional; not default.
- New paired-direction limited-half search report32: .649348s versus
  Sassy13.274224s =20.442×, counts/signatures all matched.
- Paired screen10k: .103559s versus5.597173s =54.05× warm, BUT first run
  had page faults and whole native process took30.696s. Do not claim cold10×.
- Forward production CLI screen1k completed1000 witnesses,2.256s; report32
  completed78,733 sites,3.933s (3.044s search+output). These are not paired
  direction timings yet. CLI audit verifies unique intervals, genomic block
  lengths and counts/signatures against both benchmark engines.

## Current live job (revalidate process before acting)

PID15349 runner, PID15351 Sassy at17:06:40UTC:
`fm-two-directions-sites1000`, 1 repetition, timeout600s perengine.
Started17:04:24; `results/fm-two-directions-1000.log`.
Do not compile/run overlapping EC2 timing work while it is active.
After it terminates transfer/build current local source and run next matrix.

## Indexes retained

- data/fm-full-v1 (13x200Mb), old benchmark manifest, no provenance info.
- data/fm-full-1300m (2 shards), old benchmark manifest.
- data/fm-production-v1 (forward2shards, source SHA/FASTA offsets),148.828s build.
- data/fm-reverse-v1 (same records individually REVERSED, not complemented),
  136.946s build, source provenance retained. Both indexes together~23GB.

The reverse index changes algorithm traversal only: reverse both target pattern
and reference text, then map coordinates back. ASO/RNA biological orientation
is unchanged. User asked this; explanation delivered. It adds no opposite-strand
hits. At k<=3 one query half has<=floor(k/2) edits; union limited-half searches
from both ends covers all intervals. Scalar oracles validate k0..3 including
mixed edits/repeats/boundaries, and fixture CLI checks reverse coordinate maps.

## Current local changes not yet transferred/built on EC2

- `src/indexed.rs`: optional `attach_reverse_immutable`, paired search coalesces
  intervals, still scalar-verifies every source interval before annotations.
- `src/main.rs`: optional `--reverse-index` and manifest hash.
- `tests/indexed_cli.rs`: paired annotated outputs equal ordinary outputs k0..3;
  reverse-only primary index rejected. Passed.
- `benchmarks/run_cli_iteration.py`: optional --reverse-index, retained JSONL
  output/timing/provenance and CLI_ITERATIONS.md. Ready to time paired CLI.
- `benchmarks/engine_driver.rs`, `src/bin/ooff-index.rs`: optional `--site-tuples
  PATH` writes full (repetition,query index,record index,start,end,distance) CSV,
  with serialization/flush inside timed search. Default comparator behavior
  unchanged. Need rebuild external comparator via build_comparator_ec2.sh AFTER
  current job finishes. Native binary must also rebuild.
- `run_iteration.py --site-tuples`: passes separate artifact paths, checks full
  tuple equality AND uniqueness, retains raw tuples. Use for stronger fullhuman
  site-set validation (current earlier ratios only additive signatures).
- Latest all-target tests session23709 running/possiblydone; prior14 tests pass.
  Latest clippy session1825; read outcome. Re-run only if changed afterward.
- README/HANDOFF/reference docs updated locally, server docs stale. BENCHMARKS
  intro updated locally but table only33 rows until further artifact sync.

## Next work

1. Let live1k report finish, inspect completeness/counts/speed and failures.
2. Sync/build current code; rebuild external Sassy only in comparator dir.
3. Run report32 exact site-tuples comparison and k0..2; run paired production
   CLI report32 with retained output and audit. Record every iteration in MD.
4. Broaden report1000 and screen1k/10k realistic/mixed/stress, fair threads,
   cold versus warm cache and end-to-end output comparisons. Current all1thread.
5. Actual allele-specific6338pool unavailable: async question pending asking
   user for accessible path. Available original1k/32 and generated10k are not
   silently equivalent to allele pool.
6. Cold page-fault behavior and extra index storage are material limitations;
   keep measuring and state explicitly. Do not mark goal complete yet.
7. User expected a few hours; currently~1h20 of worker use. Remain autonomous.

Local Rust: CARGO_HOME=/tmp/ooff-cargo RUSTUP_HOME=/tmp/ooff-rustup
/tmp/ooff-cargo/bin/cargo, OMP_NUM_THREADS=2 for small tests only.
