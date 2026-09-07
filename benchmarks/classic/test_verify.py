import json,tempfile,unittest
from pathlib import Path
from verify import verify,span,distance

class VerificationTests(unittest.TestCase):
    def test_global_distance_and_cigar(self):
        self.assertEqual(distance('ACGT','CGT'),1)
        self.assertEqual(distance('ACGT','ACGGT'),1)
        self.assertEqual(span('2S17M1I'),17)
        self.assertIsNone(span('10M100N10M'))
        self.assertIsNone(span('20Mgarbage'))
    def test_strand_unknowns_alternatives_and_terminal_gaps(self):
        root=Path(tempfile.mkdtemp(prefix='ooff-classic-verify-'))
        query='ACGTGATCTAGCTACGATGC'
        text=query+'N'+query[1:]+'N'+'N'*20
        (root/'eligible.fa').write_text(text)
        (root/'records.json').write_text(json.dumps([dict(id='r0',offset=0,length=len(text))]))
        (root/'queries.fa').write_text('>q0\n'+query+'\n>q1\n'+query+'\n>q2\n'+query+'\n')
        # q0 only reverse-strand primary: ineligible. q1 valid plus-strand XA.
        # q2 has one terminal query gap, recovered by full-query scalar distance.
        sam='q0\t16\tr0\t1\t0\t20M\t*\t0\t0\t*\t*\n'
        sam+='q1\t16\tr0\t1\t0\t20M\t*\t0\t0\t*\t*\tXA:Z:r0,+1,20M,0;\n'
        sam+='q2\t0\tr0\t22\t0\t1S19M\t*\t0\t0\t*\t*\n'
        (root/'out.sam').write_text(sam)
        result=verify(root/'out.sam','sam',root/'queries.fa',root)
        self.assertEqual(set(result['witnesses']),{'q1','q2'})
        self.assertEqual(result['witnesses']['q2']['distance'],1)
        parallel=verify(root/'out.sam','sam',root/'queries.fa',root,threads=2)
        self.assertEqual(parallel['witnesses'],result['witnesses'])
        (root/'native-record-map.json').write_text(json.dumps(['r0']))
        native=dict(complete=True,mode='screen',k=3,runs=[dict(counts=[0,1,1],witness_intervals=[None,[0,0,20,0],[0,21,40,1]])])
        (root/'native.json').write_text(json.dumps(native))
        checked=verify(root/'native.json','native',root/'queries.fa',root)
        self.assertEqual(checked['witnesses'],result['witnesses'])
        native['runs'][0]['witness_intervals'][2][3]=0
        (root/'native-bad.json').write_text(json.dumps(native))
        with self.assertRaises(AssertionError):verify(root/'native-bad.json','native',root/'queries.fa',root)

        (root/'out.tsv').write_text('q0\tr0\t20\t1\nq1\tr0\t42\t61\nq2\tgb|r0|\t22\t40\n')
        result=verify(root/'out.tsv','blast',root/'queries.fa',root)
        self.assertEqual(set(result['witnesses']),{'q2'})

if __name__=='__main__':unittest.main()
