/* sample */
with recursive fish(d,d0,d1,d2,d3,d4,d5,d6,d7,d8) as (values(0,0,1,1,2,1,0,0,0,0) union all select d+1,d1,d2,d3,d4,d5,d6,d7+d0,d8,d0 from fish where d < 30) select *, d0+d1+d2+d3+d4+d5+d6+d7+d8 from fish;

// $cat input | tr , '\n' | sort | uniq -c 
/*
 212 1
  23 2
  25 3
  21 4
  19 5
 */
with recursive fish(d,d0,d1,d2,d3,d4,d5,d6,d7,d8) as (values(0,0,212,23,25,21,19,0,0,0) union all select d+1,d1,d2,d3,d4,d5,d6,d7+d0,d8,d0 from fish where d <= 80) select *, d0+d1+d2+d3+d4+d5+d6+d7+d8 from fish where d=80;

with recursive fish(d,d0,d1,d2,d3,d4,d5,d6,d7,d8) as (values(0,0,212,23,25,21,19,0,0,0) union all select d+1,d1,d2,d3,d4,d5,d6,d7+d0,d8,d0 from fish where d <= 256) select *, d0+d1+d2+d3+d4+d5+d6+d7+d8 from fish where d=256;
