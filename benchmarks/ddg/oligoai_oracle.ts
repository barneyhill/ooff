// Run the actual local OligoAI implementation as the compatibility oracle.
const [source, inputPath, offPath, length] = process.argv.slice(2);
const { computeSpecificity } = await import(source);
const inputs = (await Bun.file(inputPath).text()).trim().split('\n').filter(Boolean).map(JSON.parse);
const off = (await Bun.file(offPath).text()).split('\n').filter(l => !l.startsWith('>')).join('');
const result = await computeSpecificity(inputs.map(q => ({asoDnaSequence:q.aso,targetDnaSequence:q.target})), off, Number(length));
for (let i=0;i<inputs.length;i++) console.log(JSON.stringify({id:inputs[i].id,...result[i]}));
