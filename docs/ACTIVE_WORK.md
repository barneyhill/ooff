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
