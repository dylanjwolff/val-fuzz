# ./yourfuzzer <smtfile> <iter> <timeout> <modulo>
cswap-cli -v -w 1,1,10 -i $2 -t $3 -p 0,1,2,3,4,5,6,7,8 --const-relations 3 --skolemize-universal --abstract-domain-vars --multi-enforce 7 $1
