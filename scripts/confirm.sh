solver-cli -v $1
if [ 0 == $? ]; then
    mkdir -p confirmed
    mv $1 confirmed
else
    mkdir -p no-repro 
    mv $1 no-repro 
fi
