/* Development-only export of factual 37C energy tables. Not linked into oofft. */
#include <stdio.h>
#include <ViennaRNA/params/basic.h>
#include <ViennaRNA/params/default.h>
#define TABLE(field) table(#field, (const int *)p->field, sizeof(p->field)/sizeof(int))
static void table(const char *name, const int *data, size_t count) {
  printf("\"%s\":[", name);
  for(size_t i=0;i<count;i++) printf("%s%d",i?",":"",data[i]);
  printf("],\n");
}
int main(void) {
  vrna_param_t *p=vrna_params(NULL);
  puts("{");
  TABLE(stack); TABLE(bulge); TABLE(internal_loop);
  TABLE(mismatchExt); TABLE(mismatchI); TABLE(mismatch1nI); TABLE(mismatch23I);
  TABLE(dangle5); TABLE(dangle3); TABLE(int11); TABLE(int21); TABLE(int22); TABLE(ninio);
  printf("\"terminal_au\":%d,\"duplex_init\":%d,\"temperature\":%.1f,\"lxc\":%.12g,\"max_ninio\":%d\n}\n",
         p->TerminalAU,p->DuplexInit,p->temperature,p->lxc,MAX_NINIO);
}
