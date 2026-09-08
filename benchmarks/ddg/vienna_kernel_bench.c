/* External benchmark only: call the unmodified pinned ViennaRNA library.
 * All input parsing is outside timing; every computed energy is checked. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <omp.h>
#include <ViennaRNA/duplex.h>
typedef struct { char *a, *b; long expected; } Case;
int main(int argc, char **argv) {
  if (argc < 3) return 2;
  FILE *input=fopen(argv[1],"r"); if (!input) return 2;
  Case *cases=NULL; size_t count=0,capacity=0,buflen=0; char *line=NULL;
  while (getline(&line,&buflen,input)>0) {
    char *a=strtok(line,"\t"), *b=strtok(NULL,"\t"), *e=strtok(NULL,"\t\r\n");
    if (!a || !b || !e) return 2;
    if (count==capacity) { capacity=capacity?capacity*2:1024; cases=realloc(cases,capacity*sizeof(*cases)); if(!cases) return 2; }
    cases[count]=(Case){strdup(a),strdup(b),strtol(e,NULL,10)};
    for(char *s=cases[count].a;*s;s++) if(*s=='T') *s='U';
    for(char *s=cases[count].b;*s;s++) if(*s=='T') *s='U';
    count++;
  }
  fclose(input); free(line);
  int repetitions=atoi(argv[2]),threads=argc>3?atoi(argv[3]):1;
  if(!count || repetitions<1 || threads<1) return 2;
  omp_set_dynamic(0);
  size_t chunk=(count+threads-1)/threads;
  /* Initialise each worker's thread-local Vienna state and check every case. */
  #pragma omp parallel num_threads(threads)
  {
    size_t lo=(size_t)omp_get_thread_num()*chunk, hi=lo+chunk;
    if(hi>count)hi=count;
    for(size_t i=lo;i<hi;i++) {
      duplexT d=duplexfold(cases[i].a,cases[i].b);
      long cents=lround(d.energy*100.0); free(d.structure);
      if(cents!=cases[i].expected) { fprintf(stderr,"Vienna mismatch at %zu: %ld != %ld\n",i,cents,cases[i].expected); exit(3); }
    }
  }
  long long checksum=0;
  double start=omp_get_wtime();
  #pragma omp parallel num_threads(threads) reduction(+:checksum)
  {
    size_t lo=(size_t)omp_get_thread_num()*chunk, hi=lo+chunk;
    if(hi>count)hi=count;
    for(int r=0;r<repetitions;r++) for(size_t i=lo;i<hi;i++) {
      duplexT d=duplexfold(cases[i].a,cases[i].b);
      long cents=lround(d.energy*100.0); free(d.structure);
      if(cents!=cases[i].expected) { fprintf(stderr,"Vienna mismatch at %zu\n",i); exit(3); }
      checksum+=cents;
    }
  }
  printf("{\"complete\":true,\"cases\":%zu,\"repetitions\":%d,\"threads\":%d,\"seconds\":%.9f,\"checksum\":%lld,\"checked_each_energy\":true}\n",count,repetitions,threads,omp_get_wtime()-start,checksum);
  for(size_t i=0;i<count;i++){free(cases[i].a);free(cases[i].b);} free(cases);
  return 0;
}
