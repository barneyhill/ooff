/* Verification only: enumerate the pinned library's complete loop-energy domain. */
#include <stdint.h>
#include <stdio.h>
#include <ViennaRNA/params/basic.h>
#include <ViennaRNA/eval/internal.h>
int main(void) {
  vrna_param_t *p=vrna_params(NULL);
  for(unsigned u=0;u<=30;u++) for(unsigned v=0;v<=30-u;v++) {
    unsigned char buffer[6*6*5*5*5*5*4];
    size_t n=0;
    for(unsigned t=1;t<=6;t++) for(unsigned s=1;s<=6;s++)
    for(int a=0;a<5;a++) for(int b=0;b<5;b++) for(int c=0;c<5;c++) for(int d=0;d<5;d++) {
      uint32_t e=(uint32_t)vrna_E_internal(u,v,t,s,a,b,c,d,p);
      for(int shift=0;shift<32;shift+=8) buffer[n++]=(unsigned char)(e>>shift);
    }
    if(fwrite(buffer,1,n,stdout)!=n) return 1;
  }
  return 0;
}
